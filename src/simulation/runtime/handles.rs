mod file_system;

use std::{
    cell::RefCell,
    sync::mpsc::{Receiver, Sender},
};

use crate::simulation::runtime::file_system::{FileOp, FileResult};

#[derive(Debug, PartialEq)]
pub enum Request {
    File(FileOp),
    Shutdown,
}

#[derive(Debug)]
pub enum Response {
    File(FileResult),
    Shutdown,
}

pub struct Handle {
    pub response: Receiver<Response>,
    pub request: Sender<Request>,
}

thread_local! {
   pub  static HANDLE: RefCell<Option<Handle>> = RefCell::new(None);
}
