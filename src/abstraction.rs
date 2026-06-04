use std::{
    cell::RefCell,
    sync::mpsc::{Receiver, Sender},
};

use crate::abstraction::file_system::stub::{FileOp, FileResult};

pub mod environment;
pub mod file_system;

#[derive(Debug, PartialEq)]
enum Request {
    File(FileOp),
    Shutdown,
}

#[derive(Debug)]
enum Response {
    File(FileResult),
    Shutdown,
}

struct Handle {
    pub response: Receiver<Response>,
    pub request: Sender<Request>,
}

thread_local! {
    static HANDLE: RefCell<Option<Handle>> = RefCell::new(None);
}

fn trap(req: Request) -> Response {
    HANDLE.with_borrow_mut(|h| {
        let handle = h.as_mut().expect("handle used before initialization");
        handle
            .request
            .send(req)
            .expect("runtime terminated before replicas");
    });

    let resp = HANDLE.with_borrow_mut(|handle| handle.as_mut().unwrap().response.recv());
    resp.expect("runtime terminated before replicas")
}
