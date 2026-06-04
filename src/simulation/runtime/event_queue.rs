use std::{cmp::Ordering, collections::BinaryHeap};

use super::Event;

pub struct EventQueue {
    heap: BinaryHeap<EventOrd>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.heap.push(EventOrd(event));
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.heap.pop().map(|e| e.0)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

struct EventOrd(Event);

impl PartialEq for EventOrd {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for EventOrd {}

impl PartialOrd for EventOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap, so we reverse to get min-first
        other
            .0
            .fire_at
            .cmp(&self.0.fire_at)
            .then(other.0.priority.cmp(&self.0.priority))
            .then(other.0.sequence.cmp(&self.0.sequence))
    }
}
