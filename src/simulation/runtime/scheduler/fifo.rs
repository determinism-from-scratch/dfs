use crate::{
    abstraction::file_system::FileSystem,
    simulation::runtime::{Action, Context, scheduler::Scheduler},
};

pub struct FiFo {}

impl Scheduler for FiFo {
    fn decide<FS: FileSystem>(&mut self, cx: &Context<FS>) -> Action {
        unimplemented!()
    }
}
