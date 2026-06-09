pub mod real;
pub mod stub;

use std::fmt::Debug;
use std::io::{self};

use crate::simulation::runtime::environment::file_system::memory::OpenMode;

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

pub fn delete(path: &str) -> io::Result<()> {
    FILE_SYSTEM.with(|fs| fs.delete(path))
}

pub fn copy(path: &str) {
    FILE_SYSTEM.with(|fs| fs.copy(path))
}

enum File {
    Real(real::File),
    Stub(stub::File),
}

impl File {
    pub fn close(self) -> std::io::Result<()> {
        match self {
            File::Real(e) => e.close(),
            File::Stub(e) => e.close(),
        }
    }
    pub fn lseek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match self {
            File::Real(e) => e.lseek(pos),
            File::Stub(e) => e.lseek(pos),
        }
    }
    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            File::Real(e) => e.write(buf),
            File::Stub(e) => e.write(buf),
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            File::Real(e) => e.read(buf),
            File::Stub(e) => e.read(buf),
        }
    }
}

impl FileSystem {
    fn open(&self, path: &str, mode: OpenMode) -> io::Result<File> {
        match self {
            Self::Real(fs) => match fs.open(path, mode) {
                Ok(e) => Ok(File::Real(e)),
                Err(e) => Err(e),
            },
            Self::Stub(fs) => match fs.open(path, mode) {
                Ok(e) => Ok(File::Stub(e)),
                Err(e) => Err(e),
            },
        }
    }

    fn delete(&self, path: &str) -> io::Result<()> {
        match self {
            Self::Real(fs) => fs.delete(path),
            Self::Stub(fs) => fs.delete(path),
        }
    }
    fn copy(&self, path: &str) {
        match self {
            Self::Real(fs) => unimplemented!(),
            Self::Stub(fs) => unimplemented!(),
        }
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
//     fn read(&mut self, buf: &mut [u]) -> io::Result<usize>;
//     fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
//     fn lseek(&mut self, pos: SeekFrom) -> io::Result<u64>;
// }
