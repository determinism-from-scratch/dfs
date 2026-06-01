use crate::{
    abstraction::file_system::FileSystem,
    simulation::runtime::{Action, Context},
};

pub mod fifo;

pub trait Scheduler {
    fn decide<FS: FileSystem>(&mut self, cx: &Context<FS>) -> Action;
}
