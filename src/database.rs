use crate::abstraction::{environment::Environment, file_system::FileSystem};

pub mod btree;
#[cfg(test)]
pub mod btree_tests;

pub struct Replica<FS: FileSystem> {
    env: Environment<FS>,
}

impl<FS: FileSystem> Replica<FS> {
    pub fn run(&mut self) {
        let file = self
            .env
            .file_system
            .open("hello", crate::abstraction::file_system::OpenMode::Read)
            .unwrap();
    }
}

impl<FS: FileSystem> Replica<FS> {
    pub fn new(env: Environment<FS>) -> Self {
        Self { env }
    }
}
