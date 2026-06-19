use crate::simulation::runtime::{Event, ReplicaId, counter::Counter, replica::handles::Request};

pub struct Scheduler {
    counter: Counter,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl super::Scheduler for Scheduler {
    fn schedule(&mut self, req: Request, replica_id: ReplicaId) -> Event {
        let sequence = self.counter.next();
        Event {
            replica_id,
            req,
            fault: None,
            fire_at: 0,
            priority: 0,
            sequence,
        }
    }

    fn reschedule(&mut self, mut event: Event) -> Event {
        event.sequence = self.counter.next();
        event
    }
}
