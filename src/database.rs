pub mod btree;
#[cfg(test)]
pub mod btree_tests;

pub struct Replica {}

impl Replica {
    pub fn run(&mut self) {
        unimplemented!()
    }
    pub fn new() -> Self {
        Self {}
    }
}
