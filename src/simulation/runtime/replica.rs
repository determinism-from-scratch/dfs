pub mod environment;
pub mod handles;

use std::sync::mpsc::{Receiver, Sender};

use environment::Environment;
use handles::{Request, Response};

use crate::simulation::runtime::ReplicaId;

use super::replica::environment::file_system::FileSystem;

pub struct Replica<FS: FileSystem> {
    id: ReplicaId,
    environment: Environment<FS>,
    req_receiver: Receiver<Request>,
    resp_sender: Sender<Response>,
}

impl<FS: FileSystem> Replica<FS> {
    pub fn new(
        id: ReplicaId,
        environment: Environment<FS>,
        req_receiver: Receiver<Request>,
        resp_sender: Sender<Response>,
    ) -> Self {
        Self {
            id,
            environment,
            req_receiver,
            resp_sender,
        }
    }
    pub fn trap(&mut self, resp: Response) -> Request {
        self.resp_sender
            .send(resp)
            .expect("replica terminated unexpectitly");

        self.req_receiver
            .recv()
            .expect("replica terminated unexpectitly")
    }

    pub fn environment(&mut self) -> &mut Environment<FS> {
        &mut self.environment
    }
    pub fn id(&self) -> ReplicaId {
        self.id
    }
}
