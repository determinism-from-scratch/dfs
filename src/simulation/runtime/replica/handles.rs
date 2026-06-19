use std::{
    cell::RefCell,
    sync::mpsc::{Receiver, Sender},
};

use crate::simulation::runtime::replica::handles::file_system::{FileOp, FileResult};

#[derive(Debug, PartialEq)]
pub enum Request {
    Start,
    File(FileOp),
    Shutdown,
}

#[derive(Debug)]
pub enum Response {
    Start,
    File(FileResult),
    Shutdown,
}

pub struct Handle {
    pub response: Receiver<Response>,
    pub request: Sender<Request>,
}

thread_local! {
   pub static HANDLE: RefCell<Option<Handle>> = const { RefCell::new(None)};
}

pub mod file_system {
    use std::io::{self, SeekFrom};

    use crate::simulation::runtime::environment::file_system::memory::OpenMode;

    // use crate::abstraction::file_system::OpenMode;

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
}
