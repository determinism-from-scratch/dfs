use std::collections::BTreeMap;
use std::io::{self, SeekFrom};

use crate::simulation::runtime::{
    Event, ReplicaId,
    environment::Action,
    replica::handles::{
        Request, Response,
        file_system::{Fd, FileOp, FileResult},
    },
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OpenMode {
    Read,
    Write,
    ReadWrite,
    Append,
}

/// One open file description within a single replica's disk: the file it
/// points at, the byte offset of the read/write cursor, and the access it
/// was granted at `open` time.
///
/// The cursor may legally sit past the end of the file; a later write
/// fills the gap with zeros (sparse-file semantics), matching what the OS
/// does for the `real` backend.
struct OpenFile {
    path: String,
    cursor: u64,
    readable: bool,
    writable: bool,
    append: bool,
}

/// One replica's private, in-memory local disk.
///
/// `files` is the bytes on disk (path -> contents). `open` hands back an
/// `Fd` that indexes into `handles`; every later op resolves the handle
/// there. Both maps are `BTreeMap` so iteration order is independent of
/// any random seed — no nondeterminism can leak into a run.
#[derive(Default)]
struct Disk {
    files: BTreeMap<String, Vec<u8>>,
    handles: BTreeMap<Fd, OpenFile>,
    next_fd: Fd,
}

impl Disk {
    fn alloc_fd(&mut self) -> Fd {
        let fd = self.next_fd;
        // fds are never reused, which sidesteps ABA: a stale fd always
        // resolves to "no such handle" rather than a recycled file.
        self.next_fd = self.next_fd.checked_add(1).expect("fd space exhausted");
        fd
    }

    fn open(&mut self, path: String, mode: OpenMode) -> io::Result<Fd> {
        // Access bits mirror `real::FileSystem::open` exactly so a
        // workload cannot tell the two backends apart.
        let (readable, writable, create, truncate, append) = match mode {
            OpenMode::Read => (true, false, false, false, false),
            OpenMode::Write => (false, true, true, true, false),
            OpenMode::ReadWrite => (true, true, true, false, false),
            OpenMode::Append => (false, true, true, false, true),
        };

        match self.files.get_mut(&path) {
            Some(contents) => {
                if truncate {
                    contents.clear();
                }
            }
            None => {
                if create {
                    self.files.insert(path.clone(), Vec::new());
                } else {
                    return Err(io::Error::from(io::ErrorKind::NotFound));
                }
            }
        }

        let fd = self.alloc_fd();
        self.handles.insert(
            fd,
            OpenFile {
                path,
                cursor: 0,
                readable,
                writable,
                append,
            },
        );
        Ok(fd)
    }

    fn delete(&mut self, path: &str) -> io::Result<()> {
        // Mirrors `std::fs::remove_file`: deleting a missing path errors.
        //
        // NOTE: any fd still open on this path will start failing its
        // reads/writes. POSIX keeps the bytes alive until the last fd
        // closes; modelling that faithfully needs an inode + link-count
        // layer, which we skip for now.
        match self.files.remove(path) {
            Some(_) => Ok(()),
            None => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }

    fn read(&mut self, fd: Fd, len: usize) -> io::Result<Vec<u8>> {
        let handle = self.handles.get_mut(&fd).ok_or_else(bad_fd)?;
        if !handle.readable {
            return Err(bad_fd());
        }
        let contents = self.files.get(&handle.path).ok_or_else(bad_fd)?;

        if handle.cursor >= contents.len() as u64 {
            return Ok(Vec::new()); // at or past EOF -> 0 bytes
        }
        let start = handle.cursor as usize;
        let n = (contents.len() - start).min(len);
        let out = contents[start..start + n].to_vec();
        handle.cursor += n as u64;
        Ok(out)
    }

    fn write(&mut self, fd: Fd, data: &[u8]) -> io::Result<usize> {
        let handle = self.handles.get_mut(&fd).ok_or_else(bad_fd)?;
        if !handle.writable {
            return Err(bad_fd());
        }
        let contents = self.files.get_mut(&handle.path).ok_or_else(bad_fd)?;

        // O_APPEND: every write lands at the current end of file,
        // regardless of where the cursor was left.
        if handle.append {
            handle.cursor = contents.len() as u64;
        }

        let start = handle.cursor as usize;
        // A cursor past EOF leaves a zero-filled hole before the data.
        if start > contents.len() {
            contents.resize(start, 0);
        }
        let end = start + data.len();
        if end > contents.len() {
            contents.resize(end, 0);
        }
        contents[start..end].copy_from_slice(data);
        handle.cursor = end as u64;
        Ok(data.len())
    }

    fn seek(&mut self, fd: Fd, pos: SeekFrom) -> io::Result<u64> {
        let handle = self.handles.get_mut(&fd).ok_or_else(bad_fd)?;
        let file_len = self.files.get(&handle.path).ok_or_else(bad_fd)?.len() as u64;

        let cursor = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::End(offset) => offset_from(file_len, offset)?,
            SeekFrom::Current(offset) => offset_from(handle.cursor, offset)?,
        };
        handle.cursor = cursor;
        Ok(cursor)
    }

    fn close(&mut self, fd: Fd) -> io::Result<()> {
        match self.handles.remove(&fd) {
            Some(_) => Ok(()),
            None => Err(bad_fd()),
        }
    }
}

/// The simulator's in-memory filesystem: the single environment backend
/// shared by *every* replica.
///
/// Storage is **not** shared between replicas. Each replica gets its own
/// isolated [`Disk`], created lazily the first time that replica issues a
/// file op (`entry(..).or_default()`). Replicas therefore see nothing of
/// one another through the filesystem — they coordinate over the network —
/// exactly like nodes with independent local storage in a real
/// distributed deployment. Each disk also has its own `Fd` space, so the
/// same numeric fd means different things on different replicas.
#[derive(Default)]
pub struct FileSystem {
    disks: BTreeMap<ReplicaId, Disk>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            disks: BTreeMap::new(),
        }
    }
}

impl super::FileSystem for FileSystem {
    fn serve(&mut self, event: Event) -> Action {
        let replica_id = event.replica_id;
        let file_op = match event.req {
            Request::File(op) => op,
            other => panic!("non-file request routed to the file system: {other:?}"),
        };

        // First contact from a replica conjures its (empty) disk.
        let disk = self.disks.entry(replica_id).or_default();

        let result = match file_op {
            FileOp::Open { path, mode } => FileResult::Open(disk.open(path, mode)),
            FileOp::Delete { path } => FileResult::Delete(disk.delete(&path)),
            FileOp::Read { fd, len } => FileResult::Read(disk.read(fd, len)),
            FileOp::Write { fd, data } => FileResult::Write(disk.write(fd, &data)),
            FileOp::Seek { fd, pos } => FileResult::Seek(disk.seek(fd, pos)),
            FileOp::Close { fd } => FileResult::Close(disk.close(fd)),
        };

        Action::Run(Response::File(result))
    }
}

/// Applies a signed seek delta to an unsigned base. Returns an error for
/// a result that would land before byte 0 (or overflow `u64`), matching
/// `std::io::Seek`.
fn offset_from(base: u64, offset: i64) -> io::Result<u64> {
    base.checked_add_signed(offset).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid seek to a negative position",
        )
    })
}

fn bad_fd() -> io::Error {
    io::Error::other("bad file descriptor")
}
