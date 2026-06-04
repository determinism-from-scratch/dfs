use std::{
    cell::{RefCell, RefMut},
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
    sync::mpsc::{Receiver, Sender},
};

use crate::simulation::runtime::{
    handles::{Request, Response},
    scheduler::Scheduler,
};

mod clock;
pub mod handles;
mod scheduler;

struct Runtime<S: Scheduler, F: FaultInjector> {
    scheduler: S,
    fault_injector: F,
    queue: EventQueue,
    replicas: Vec<Replica>,
}

struct Replica {
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

impl<S: Scheduler, F: FaultInjector> Runtime<S, F> {
    pub fn run(&mut self) {
        while !self.done() {
            // this is the scheduling - use the sequence as a eal breaker
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

struct Event {
    replica_id: usize,
    req: Request,
    fault: Option<Fault>,
    fire_at: u64,
    priority: u64,
    sequence: u64,
}

enum Fault {
    None,
    Corrupted,
}

trait FaultInjector {
    fn inject(&mut self, event: Event) -> Event;
}

struct Environment {}

impl Environment {
    pub fn serve(&mut self, event: Event) -> Action {
        unimplemented!()
    }
}

enum Action {
    Run(Response),
    Requeue(Event),
}

trait FileSystem {
    fn service() -> Action;
}

struct EventQueue {
    heap: BinaryHeap<EventOrd>,
}

impl EventQueue {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    fn push(&mut self, event: Event) {
        self.heap.push(EventOrd(event));
    }

    fn pop(&mut self) -> Option<Event> {
        self.heap.pop().map(|e| e.0)
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

struct EventOrd(Event);

impl PartialEq for EventOrd {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for EventOrd {}

impl PartialOrd for EventOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap, so we reverse to get min-first
        other
            .0
            .fire_at
            .cmp(&self.0.fire_at)
            .then(other.0.priority.cmp(&self.0.priority))
            .then(other.0.sequence.cmp(&self.0.sequence))
    }
}
