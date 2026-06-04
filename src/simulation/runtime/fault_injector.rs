use super::Event;

pub enum Fault {
    None,
    Corrupted,
}

pub trait FaultInjector {
    fn inject(&mut self, event: Event) -> Event;
}
