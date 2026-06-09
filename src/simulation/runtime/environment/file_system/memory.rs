use crate::simulation::runtime::{
    Event,
    environment::Action,
    replica::handles::{Response, file_system::FileResult},
};

pub struct FileSystem {}

impl super::FileSystem for FileSystem {
    fn serve(&mut self, event: Event) -> Action {
        Action::Run(Response::File(FileResult::Open(Ok(42))))
    }
}
