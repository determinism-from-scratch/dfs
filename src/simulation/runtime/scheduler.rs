pub mod fifo;

use crate::simulation::runtime::ReplicaId;

use super::Event;
use super::replica::handles::Request;

pub trait Scheduler {
    fn schedule(&mut self, req: Request, replica_id: ReplicaId) -> Event;

    fn reschedule(&mut self, event: Event) -> Event;
}
