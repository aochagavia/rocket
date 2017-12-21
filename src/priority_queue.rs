use std::collections::BinaryHeap;
use std::time::Duration;
use std::cmp::Ordering;
use std::fmt;

use ApplicationState;

/// TODO: docs
// This is a Priority Queue. Rust's native implementation of a BinaryHeap is a max-heap - we want to
// use a min-heap, so we have to define a min-ordering for our custom `min` type which is just an
// alias for `u64`.

/// A ScheduledEvent is a tuple of a `Duration` (time) and `fn(&mut ApplicationState)` (handler).
struct ScheduledEvent(Duration, fn(&mut ApplicationState));

impl fmt::Debug for ScheduledEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScheduledEvent: {:?}", self.0)
    }
}

impl Eq for ScheduledEvent {}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &ScheduledEvent) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &ScheduledEvent) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord,
        }
    }
}

#[derive(Debug)]
pub struct PriorityQueue {
    heap: BinaryHeap<ScheduledEvent>
}

// Thin wrapper around our BinaryHeap<ScheduledEvent> so we don't have to use the `ScheduledEvent` type 
// elsewhere in the code.
impl PriorityQueue {
    pub fn new() -> PriorityQueue {
        PriorityQueue {
            heap: BinaryHeap::new()
        }
    }

    pub fn peek(&self) -> Option<Duration> {
        if let Some(&ScheduledEvent(time, _handler)) = self.heap.peek() {
            Some(time)
        } else {
            None
        }
    }

    pub fn push(&mut self, time: Duration, handler: fn(&mut ApplicationState)) {
        self.heap.push(ScheduledEvent(time, handler));
    }

    pub fn pop(&mut self) -> Option<(Duration, fn(&mut ApplicationState))> {
        if let Some(ScheduledEvent(time, handler)) = self.heap.pop() {
            Some((time, handler))
        } else {
            None
        }
    }
}
