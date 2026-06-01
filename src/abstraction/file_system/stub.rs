
use std::collections::BTreeMap;
use std::io::{self, SeekFrom};
use std::sync::{Arc, Mutex};

use crate::abstraction::file_system::{self, OpenMode};

type Inode = Arc<Mutex<Vec<u8>>>;

#[derive(Clone, Default, Debug)]
pub struct FileSystem {
    /// Cloning a `MemFs` clones the Arc — both handles see the same files.
    /// Construct fresh `MemFs::new()` instances when you want isolated FSes.
    inner: Arc<Mutex<BTreeMap<String, Inode>>>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Test helper: list every path currently present.
    pub fn list(&self) -> Vec<String> {
        self.inner.lock().unwrap().keys().cloned().collect()
    }

    /// Test helper: snapshot a path's contents (useful for assertions).
    pub fn snapshot(&self, path: &str) -> Option<Vec<u8>> {
        let fs = self.inner.lock().unwrap();
        fs.get(path).map(|i| i.lock().unwrap().clone())
    }
}

impl file_system::FileSystem for FileSystem {
    type File = File;

    fn open(&self, path: &str, mode: OpenMode) -> io::Result<File> {
        // Hold the FS lock only long enough to clone out the Arc handle
        // to the file's contents.
        let inode: Inode = {
            let mut fs = self.inner.lock().unwrap();
            match mode {
                OpenMode::Read => fs
                    .get(path)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, path.to_string()))?
                    .clone(),
                OpenMode::Write => {
                    let inode = fs
                        .entry(path.to_string())
                        .or_insert_with(|| Arc::new(Mutex::new(Vec::new())))
                        .clone();
                    inode.lock().unwrap().clear(); // truncate
                    inode
                }
                OpenMode::ReadWrite | OpenMode::Append => fs
                    .entry(path.to_string())
                    .or_insert_with(|| Arc::new(Mutex::new(Vec::new())))
                    .clone(),
            }
        };

        let (can_read, can_write, append) = match mode {
            OpenMode::Read => (true, false, false),
            OpenMode::Write => (false, true, false),
            OpenMode::ReadWrite => (true, true, false),
            OpenMode::Append => (false, true, true),
        };

        let cursor = if append {
            inode.lock().unwrap().len() as u64
        } else {
            0
        };

        Ok(File {
            contents: inode,
            cursor,
            can_read,
            can_write,
            append,
        })
    }

    fn delete(&self, path: &str) -> io::Result<()> {
        let mut fs = self.inner.lock().unwrap();
        if fs.remove(path).is_none() {
            return Err(io::Error::new(io::ErrorKind::NotFound, path.to_string()));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct File {
    contents: Inode,
    cursor: u64,
    can_read: bool,
    can_write: bool,
    append: bool,
}

impl file_system::File for File {
    fn close(self) -> io::Result<()> {
        Ok(())
    }

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.can_read {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "not opened for read",
            ));
        }
        let data = self.contents.lock().unwrap();
        let start = self.cursor as usize;
        if start >= data.len() {
            return Ok(0); // EOF
        }
        let n = std::cmp::min(buf.len(), data.len() - start);
        buf[..n].copy_from_slice(&data[start..start + n]);
        self.cursor += n as u64;
        Ok(n)
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !self.can_write {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "not opened for write",
            ));
        }
        let mut data = self.contents.lock().unwrap();
        if self.append {
            // O_APPEND: every write goes to current EOF, atomically wrt
            // other writers on the same inode.
            self.cursor = data.len() as u64;
        }
        let start = self.cursor as usize;
        // Sparse semantics: writing past EOF zero-fills the hole.
        if start > data.len() {
            data.resize(start, 0);
        }
        let end = start + buf.len();
        if end > data.len() {
            data.resize(end, 0);
        }
        data[start..end].copy_from_slice(buf);
        self.cursor = end as u64;
        Ok(buf.len())
    }

    fn lseek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let len = self.contents.lock().unwrap().len() as u64;
        let new = match pos {
            SeekFrom::Start(n) => n,
            SeekFrom::End(d) => offset(len, d)?,
            SeekFrom::Current(d) => offset(self.cursor, d)?,
        };
        self.cursor = new;
        Ok(new)
    }
}

fn offset(base: u64, delta: i64) -> io::Result<u64> {
    let s = base as i128 + delta as i128;
    if s < 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "seek before start of file",
        ));
    }
    if s > u64::MAX as i128 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "seek past u64 max",
        ));
    }
    Ok(s as u64)
}
