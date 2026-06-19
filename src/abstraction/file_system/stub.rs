use std::{io, panic};

use crate::{
    abstraction::{file_system, trap},
    simulation::runtime::replica::handles::{
        Request, Response,
        file_system::{Fd, FileOp, FileResult},
    },
};

pub struct FileSystem {}

impl FileSystem {
    pub fn open(&self, path: &str, mode: file_system::OpenMode) -> std::io::Result<File> {
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
                io::Result::Ok(fd) => io::Result::Ok(File::new(fd)),
                io::Result::Err(err) => io::Result::Err(err),
            },
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }
    pub fn delete(&self, path: &str) -> std::io::Result<()> {
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

#[derive(Debug)]
pub struct File {
    fd: Fd,
}

impl File {
    pub fn new(fd: Fd) -> Self {
        Self { fd }
    }
}

impl File {
    pub fn close(self) -> std::io::Result<()>
    where
        Self: Sized,
    {
        let req = Request::File(FileOp::Close { fd: self.fd });
        match trap(req) {
            Response::File(FileResult::Close(res)) => res,
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
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

    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let req = Request::File(FileOp::Write {
            fd: self.fd,
            data: buf.to_vec(),
        });
        match trap(req) {
            Response::File(FileResult::Write(res)) => res,
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }

    pub fn lseek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let req = Request::File(FileOp::Seek { fd: self.fd, pos });

        match trap(req) {
            Response::File(FileResult::Seek(res)) => res,
            wrong => panic!("runtime broke protocol {:?}", wrong),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        io::SeekFrom,
        sync::mpsc::{Receiver, Sender, channel},
    };

    use crate::{
        abstraction::file_system::OpenMode,
        simulation::runtime::replica::handles::{HANDLE, Handle},
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

    #[test]
    fn open_error() {
        let (resp_sender, req_receiver) = init();
        let fs = super::FileSystem {};

        resp_sender
            .send(Response::File(FileResult::Open(Err(io::Error::from(
                io::ErrorKind::NotFound,
            )))))
            .unwrap();

        let file = fs.open("missing", OpenMode::Read);

        let req = req_receiver.try_recv().unwrap();
        assert_eq!(
            req,
            Request::File(FileOp::Open {
                path: String::from("missing"),
                mode: OpenMode::Read,
            })
        );
        let err = file.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn delete() {
        let (resp_sender, req_receiver) = init();
        let fs = super::FileSystem {};

        resp_sender
            .send(Response::File(FileResult::Delete(Ok(()))))
            .unwrap();

        let res = fs.delete("test");

        let req = req_receiver.try_recv().unwrap();
        assert_eq!(
            req,
            Request::File(FileOp::Delete {
                path: String::from("test"),
            })
        );
        assert!(res.is_ok());
    }

    #[test]
    fn read() {
        let (resp_sender, req_receiver) = init();
        let mut file = File::new(0);

        resp_sender
            .send(Response::File(FileResult::Read(Ok(vec![1, 2, 3, 4]))))
            .unwrap();

        let mut buf = [0u8; 4];
        let n = file.read(&mut buf).unwrap();

        let req = req_receiver.try_recv().unwrap();
        assert_eq!(req, Request::File(FileOp::Read { fd: 0, len: 4 }));
        assert_eq!(n, 4);
        assert_eq!(&buf, &[1, 2, 3, 4]);
    }

    #[test]
    fn read_partial() {
        // Runtime returns fewer bytes than the buffer can hold.
        let (resp_sender, req_receiver) = init();
        let mut file = File::new(0);

        resp_sender
            .send(Response::File(FileResult::Read(Ok(vec![9, 9]))))
            .unwrap();

        let mut buf = [0u8; 8];
        let n = file.read(&mut buf).unwrap();

        let req = req_receiver.try_recv().unwrap();
        assert_eq!(req, Request::File(FileOp::Read { fd: 0, len: 8 }));
        assert_eq!(n, 2);
        assert_eq!(&buf[..2], &[9, 9]);
        assert_eq!(&buf[2..], &[0; 6]); // tail left untouched
    }

    #[test]
    fn read_error() {
        let (resp_sender, req_receiver) = init();
        let mut file = File::new(7);

        resp_sender
            .send(Response::File(FileResult::Read(Err(io::Error::from(
                io::ErrorKind::UnexpectedEof,
            )))))
            .unwrap();

        let mut buf = [0u8; 8];
        let res = file.read(&mut buf);

        let req = req_receiver.try_recv().unwrap();
        assert_eq!(req, Request::File(FileOp::Read { fd: 7, len: 8 }));
        assert_eq!(res.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
        assert_eq!(&buf, &[0; 8]); // nothing copied on error
    }

    #[test]
    fn write() {
        let (resp_sender, req_receiver) = init();
        let mut file = File::new(0);

        resp_sender
            .send(Response::File(FileResult::Write(Ok(3))))
            .unwrap();

        let n = file.write(&[1, 2, 3]).unwrap();

        let req = req_receiver.try_recv().unwrap();
        assert_eq!(
            req,
            Request::File(FileOp::Write {
                fd: 0,
                data: vec![1, 2, 3],
            })
        );
        assert_eq!(n, 3);
    }

    #[test]
    fn lseek() {
        let (resp_sender, req_receiver) = init();
        let mut file = File::new(0);

        resp_sender
            .send(Response::File(FileResult::Seek(Ok(128))))
            .unwrap();

        let pos = file.lseek(SeekFrom::Start(128)).unwrap();

        let req = req_receiver.try_recv().unwrap();
        assert_eq!(
            req,
            Request::File(FileOp::Seek {
                fd: 0,
                pos: SeekFrom::Start(128),
            })
        );
        assert_eq!(pos, 128);
    }

    #[test]
    fn close() {
        let (resp_sender, req_receiver) = init();
        let file = File::new(0);

        resp_sender
            .send(Response::File(FileResult::Close(Ok(()))))
            .unwrap();

        let res = file.close();

        let req = req_receiver.try_recv().unwrap();
        assert_eq!(req, Request::File(FileOp::Close { fd: 0 }));
        assert!(res.is_ok());
    }

    #[test]
    #[should_panic(expected = "runtime broke protocol")]
    fn protocol_mismatch() {
        // Runtime answers an Open with a Delete response — the stub must reject it.
        let (resp_sender, _req_receiver) = init();
        let fs = super::FileSystem {};

        resp_sender
            .send(Response::File(FileResult::Delete(Ok(()))))
            .unwrap();

        let _ = fs.open("test", OpenMode::Read);
    }
}
