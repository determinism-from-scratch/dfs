use crate::simulation::runtime::{Event, replica::environment::Action};

pub mod memory;

pub trait FileSystem {
    fn serve(&mut self, event: Event) -> Action;
}
