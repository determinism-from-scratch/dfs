// pub mod real;
// pub mod stub;

use std::fmt::Debug;
use std::io::{self};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OpenMode {
    Read,
    Write,
    ReadWrite,
    Append,
}

thread_local! {
    static FILE_SYSTEM: FileSystem = FileSystem::Real(real::FileSystem::new(String::from("")).unwrap());
}

pub enum FileSystem {
    Real(real::FileSystem),
    Stub(stub::FileSystem),
}

pub fn open(path: &str, mode: OpenMode) -> std::io::Result<File> {
    FILE_SYSTEM.with(|fs| fs.open(path, mode))
}

enum File {
    Real(real::File),
    Stub(stub::File),
}

impl FileSystem {
    fn open(&self, path: &str, mode: OpenMode) -> io::Result<File> {
        unimplemented!()
        // match self {
        //     Self::Real(fs) => fs.open(path, mode),
        //     Self::Stub(fs) => fs.open(path, mode),
        // }
    }
}

// pub trait FileSystem: Send + Sync {
//     type File: File;
//
//     fn open(&self, path: &str, mode: OpenMode) -> io::Result<Self::File>;
//     fn delete(&self, path: &str) -> io::Result<()>;
// }
//
// pub trait File: Send {
//     /// Explicit close. `Drop` also closes; the explicit form lets
//     /// implementations surface deferred errors (buffered flushes,
//     /// fsync failures, late allocation on NFS, etc.).
//     fn close(self) -> io::Result<()>
//     where
//         Self: Sized;
//
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
//     fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
//     fn lseek(&mut self, pos: SeekFrom) -> io::Result<u64>;
// }
