use std::io::{self, SeekFrom};

use crate::abstraction::file_system::OpenMode;

struct FileSystem {}

pub type Fd = u32;

#[derive(Debug, PartialEq)]
pub enum FileOp {
    Open { path: String, mode: OpenMode },
    Delete { path: String },
    Read { fd: Fd, len: usize },
    Write { fd: Fd, data: Vec<u8> },
    Seek { fd: Fd, pos: SeekFrom },
    Close { fd: Fd },
}

#[derive(Debug)]
pub enum FileResult {
    Open(io::Result<Fd>),
    Delete(io::Result<()>),
    Read(io::Result<Vec<u8>>),
    Write(io::Result<usize>),
    Seek(io::Result<u64>),
    Close(io::Result<()>),
}
