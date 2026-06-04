// pub mod fifo;

use super::Event;
use super::replica::handles::Request;

pub trait Scheduler {
    fn schedule(&mut self, req: Request) -> Event {
        unimplemented!()
    }

    fn reschedule(&mut self, event: Event) -> Event {
        unimplemented!()
    }
}
