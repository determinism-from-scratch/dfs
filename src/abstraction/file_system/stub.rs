use std::{
    io::{self, SeekFrom},
    panic,
};

use crate::abstraction::{
    Request, Response,
    file_system::{self, OpenMode},
    trap,
};

type Fd = u32;

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

pub struct FileSystem {}

impl file_system::FileSystem for FileSystem {
    type File = File;
    fn open(&self, path: &str, mode: file_system::OpenMode) -> std::io::Result<Self::File> {
        let req = Request::File(FileOp::Open {
            path: String::from(path),
            mode,
        });
        let res = match trap(req) {
            Response::File(res) => res,
            wrong => panic!("runtime broke protocol {:?}", wrong),
        };
        match res {
            FileResult::Open(res) => match res {
                io::Result::Ok(fd) => io::Result::Ok(Self::File::new(fd)),
                io::Result::Err(err) => io::Result::Err(err),
            },
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }
    fn delete(&self, path: &str) -> std::io::Result<()> {
        let req = Request::File(FileOp::Delete {
            path: String::from(path),
        });
        let res = match trap(req) {
            Response::File(res) => res,
            wrong => panic!("runtime broke protocol {:?}", wrong),
        };
        match res {
            FileResult::Delete(res) => res,
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }
}

pub struct File {
    fd: Fd,
}

impl File {
    pub fn new(fd: Fd) -> Self {
        Self { fd }
    }
}

impl file_system::File for File {
    fn close(self) -> std::io::Result<()>
    where
        Self: Sized,
    {
        let req = Request::File(FileOp::Close { fd: self.fd });
        match trap(req) {
            Response::File(FileResult::Close(res)) => res,
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let req = Request::File(FileOp::Read {
            fd: self.fd,
            len: buf.len(),
        });
        match trap(req) {
            Response::File(FileResult::Read(res)) => match res {
                io::Result::Ok(value) => {
                    let n = value.len();
                    buf[..n].copy_from_slice(&value);
                    io::Result::Ok(n)
                }
                io::Result::Err(err) => io::Result::Err(err),
            },
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let req = Request::File(FileOp::Write {
            fd: self.fd,
            data: buf.iter().map(|n| *n).collect(),
        });
        match trap(req) {
            Response::File(FileResult::Write(res)) => res,
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }

    fn lseek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let req = Request::File(FileOp::Seek {
            fd: self.fd,
            pos: pos,
        });

        match trap(req) {
            Response::File(FileResult::Seek(res)) => res,
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::{Receiver, Sender, channel};

    use crate::abstraction::{
        HANDLE, Handle,
        file_system::{FileSystem, OpenMode},
    };

    fn init() -> (Sender<Response>, Receiver<Request>) {
        let (req_sender, req_receiver) = channel::<Request>();
        let (resp_sender, resp_receiver) = channel::<Response>();

        init_handel(req_sender, resp_receiver);
        (resp_sender, req_receiver)
    }

    fn init_handel(sender: Sender<Request>, receiver: Receiver<Response>) {
        HANDLE.with_borrow_mut(|handle| {
            *handle = Some(Handle {
                request: sender,
                response: receiver,
            })
        })
    }

    use super::*;
    #[test]
    fn open() {
        // Setup
        let (resp_sender, req_receiver) = init();
        let fs = super::FileSystem {};

        // Prepare response
        resp_sender
            .send(Response::File(FileResult::Open(Ok(0))))
            .unwrap();
        // Make Request and send it
        let file = fs.open("test", OpenMode::Read);
        // Validate request came
        let req = req_receiver.try_recv().unwrap();
        assert_eq!(
            req,
            Request::File(FileOp::Open {
                path: String::from("test"),
                mode: OpenMode::Read
            })
        );
        let file = file.unwrap();
        assert_eq!(file.fd, 0);
    }
}
