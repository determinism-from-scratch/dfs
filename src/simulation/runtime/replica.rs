pub mod environment;
pub mod handles;

use std::sync::mpsc::{Receiver, Sender};

use environment::Environment;
use handles::{Request, Response};

pub struct Replica {
    environment: Environment,
    req_receiver: Receiver<Request>,
    resp_sender: Sender<Response>,
}

impl Replica {
    pub fn trap(&mut self, resp: Response) -> Request {
        unimplemented!()
    }

    pub fn environment(&mut self) -> &mut Environment {
        &mut self.environment
    }
}
