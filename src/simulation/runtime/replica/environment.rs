pub mod file_system;

use super::super::Event;
use super::handles::Response;

pub enum Action {
    Run(Response),
    Requeue(Event),
}

pub trait Environment {
    fn serve(&mut self, event: Event) -> Action {
        unimplemented!()
    }
}
