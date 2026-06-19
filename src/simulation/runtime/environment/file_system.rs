use crate::simulation::runtime::{Event, environment::Action};

pub mod memory;

pub trait FileSystem {
    fn serve(&mut self, event: Event) -> Action;
}
