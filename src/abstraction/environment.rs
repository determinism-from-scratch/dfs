use crate::abstraction::file_system::FileSystem;

pub struct Environment {
    pub file_system: FileSystem,
}

impl<FS: FileSystem> Environment<FS> {
    pub fn new(file_system: FS) -> Self {
        Self { file_system }
    }
}
