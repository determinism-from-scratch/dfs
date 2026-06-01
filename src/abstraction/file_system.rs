
pub mod real;
pub mod stub;

use std::fmt::Debug;
use std::io::{self, SeekFrom};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OpenMode {
    Read,
    Write,
    ReadWrite,
    Append,
}

pub trait FileSystem: Send + Sync {
    type File: File;

    fn open(&self, path: &str, mode: OpenMode) -> io::Result<Self::File>;
    fn delete(&self, path: &str) -> io::Result<()>;
}

pub trait File: Send + Debug {
    /// Explicit close. `Drop` also closes; the explicit form lets
    /// implementations surface deferred errors (buffered flushes,
    /// fsync failures, late allocation on NFS, etc.).
    fn close(self) -> io::Result<()>
    where
        Self: Sized;

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn lseek(&mut self, pos: SeekFrom) -> io::Result<u64>;
}
