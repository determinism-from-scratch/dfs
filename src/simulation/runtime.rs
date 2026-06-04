pub mod event_queue;
pub mod fault_injector;
pub mod replica;
mod scheduler;

use std::{
    cell::{RefCell, RefMut},
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
    sync::mpsc::{Receiver, Sender},
};

use event_queue::EventQueue;
use fault_injector::{Fault, FaultInjector};
use replica::{
    Replica,
    environment::Action,
    handles::{Request, Response},
};
use scheduler::Scheduler;

struct Event {
    pub replica_id: usize,
    pub req: Request,
    pub fault: Option<Fault>,
    pub fire_at: u64,
    pub priority: u64,
    pub sequence: u64,
}

struct Runtime<S: Scheduler, F: FaultInjector> {
    scheduler: S,
    fault_injector: F,
    queue: EventQueue,
    replicas: Vec<Replica>,
}

impl<S: Scheduler, F: FaultInjector> Runtime<S, F> {
    pub fn run(&mut self) {
        while !self.done() {
            let next = self.queue.pop().unwrap();
            let next = self.fault_injector.inject(next);
            let replica = self
                .replicas
                .get_mut(next.replica_id)
                .expect("replica needed for handling request not existing");

            let resp = match replica.environment().serve(next) {
                Action::Requeue(event) => {
                    let event = self.scheduler.reschedule(event);
                    self.queue.push(event);
                    continue;
                }
                Action::Run(resp) => resp,
            };

            let req = replica.trap(resp);
            let new = self.scheduler.schedule(req);
            self.queue.push(new);
        }
    }

    fn done(&self) -> bool {
        unimplemented!()
    }
}
