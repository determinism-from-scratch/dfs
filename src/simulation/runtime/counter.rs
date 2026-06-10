pub struct Counter {
    count: u64,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u64 {
        let id = self.count;
        self.count = self.count.checked_add(1).expect("counter overflow");
        id
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
