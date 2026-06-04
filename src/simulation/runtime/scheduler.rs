// pub mod fifo;

use crate::simulation::runtime::{Event, handles::Request};

pub trait Scheduler {
    fn schedule(&mut self, req: Request) -> Event {
        unimplemented!()
    }

    fn reschedule(&mut self, event: Event) -> Event {
        unimplemented!()
    }
}
