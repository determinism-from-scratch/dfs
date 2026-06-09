pub mod file_system;

use crate::simulation::runtime::Event;
use crate::simulation::runtime::replica::handles::{Request, Response};

use file_system::FileSystem;

pub enum Action {
    Run(Response),
    Requeue(Event),
    Remove,
}

pub struct Environment<FS: FileSystem> {
    file_system: FS,
}

impl<FS: FileSystem> Environment<FS> {
    pub fn new(file_system: FS) -> Self {
        Self { file_system }
    }
    pub fn serve(&mut self, event: Event) -> Action {
        match event.req {
            Request::Start => Action::Run(Response::Start),
            Request::File(_) => self.file_system.serve(event),
            Request::Shutdown => Action::Remove,
            _ => unimplemented!(),
        }
    }
}
