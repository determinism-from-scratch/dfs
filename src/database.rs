pub mod btree;
#[cfg(test)]
pub mod btree_tests;
pub mod file;
pub mod page;

pub struct Replica {}

impl Replica {
    pub fn run(&mut self) {
        unimplemented!()
    }
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Replica {
    fn default() -> Self {
        Self::new()
    }
}
