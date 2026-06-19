use super::Event;

pub struct FaultInjector {}

impl super::FaultInjector for FaultInjector {
    fn inject(&mut self, mut event: Event) -> Event {
        event.fault = Some(super::Fault::None);
        event
    }
}
