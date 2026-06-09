pub mod handles;

use std::sync::mpsc::{Receiver, Sender};

use handles::{Request, Response};

use crate::simulation::runtime::ReplicaId;

pub struct Replica {
    id: ReplicaId,
    req_receiver: Receiver<Request>,
    resp_sender: Sender<Response>,
}

impl Replica {
    pub fn new(
        id: ReplicaId,
        req_receiver: Receiver<Request>,
        resp_sender: Sender<Response>,
    ) -> Self {
        Self {
            id,
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

    pub fn id(&self) -> ReplicaId {
        self.id
    }
}
