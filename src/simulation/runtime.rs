pub mod counter;
pub mod environment;
pub mod event_queue;
pub mod fault_injector;
pub mod replica;
pub mod scheduler;

use std::{sync::mpsc::channel, thread};

use event_queue::EventQueue;
use fault_injector::{Fault, FaultInjector};
use replica::{
    Replica,
    handles::{Request, Response},
};
use scheduler::Scheduler;

use crate::simulation::runtime::{
    environment::{Action, Environment, file_system::FileSystem},
    replica::handles::{HANDLE, Handle},
};

type ReplicaId = usize;
type TimeStamp = u64;

pub struct Event {
    pub replica_id: ReplicaId,
    pub req: Request,
    pub fault: Option<Fault>,
    pub fire_at: TimeStamp,
    pub priority: u64,
    pub sequence: u64,
}

pub struct Runtime<S: Scheduler, F: FaultInjector, FS: FileSystem> {
    scheduler: S,
    fault_injector: F,
    environment: Environment<FS>,
    queue: EventQueue,
    replicas: Vec<Replica>,
}

impl<S: Scheduler, F: FaultInjector, FS: FileSystem> Runtime<S, F, FS> {
    pub fn new(scheduler: S, fault_injector: F, environment: Environment<FS>) -> Self {
        Self {
            scheduler,
            fault_injector,
            environment,
            queue: EventQueue::new(),
            replicas: Vec::new(),
        }
    }

    pub fn spawn<W>(&mut self, workload: W)
    where
        W: FnOnce() + Send + 'static,
    {
        let id: ReplicaId = self.replicas.len();

        // Two one-way channels form the rendezvous with the replica's thread:
        //   request : replica -> runtime
        //   response: runtime -> replica
        let (req_sender, req_receiver) = channel::<Request>();
        let (resp_sender, resp_receiver) = channel::<Response>();

        // The replica's program runs on its own thread. It installs its end of
        // the channels into the thread-local HANDLE that the stub filesystem's
        // `trap` reads, then runs the workload.
        thread::spawn(move || {
            HANDLE.with_borrow_mut(|h| {
                *h = Some(Handle {
                    request: req_sender,
                    response: resp_receiver,
                });
            });

            HANDLE.with_borrow_mut(|h| {
                let resp = h.as_ref().unwrap().response.recv().unwrap();
                match resp {
                    Response::Start => {}
                    _ => panic!("runtime broke protocol"),
                }
            });

            workload();

            HANDLE.with_borrow_mut(|h| {
                let _ = h.as_ref().unwrap().request.send(Request::Shutdown);
            });
        });

        let event = self.scheduler.schedule(Request::Start, id);
        self.queue.push(event);

        self.replicas
            .push(Replica::new(id, req_receiver, resp_sender));
    }

    pub fn run(&mut self) {
        while !self.done() {
            let next = self.queue.pop().unwrap();
            let next = self.fault_injector.inject(next);
            let replica = self
                .replicas
                .get_mut(next.replica_id)
                .expect("replica needed for handling request not existing");

            let resp = match self.environment.serve(next) {
                Action::Requeue(event) => {
                    let event = self.scheduler.reschedule(event);
                    self.queue.push(event);
                    continue;
                }
                Action::Run(resp) => resp,
                Action::Remove => continue,
            };

            let req = replica.trap(resp);
            let new = self.scheduler.schedule(req, replica.id());
            self.queue.push(new);
        }
    }

    fn done(&self) -> bool {
        self.queue.is_empty()
    }
}
