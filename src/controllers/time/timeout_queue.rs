use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;
use std::time::Duration;

use controllers::time::Timeout;

// This is a Priority Queue. Rust's native implementation of a BinaryHeap is a max-heap - we want to
// use a min-heap, so we have to define a min-ordering for a custom type which is just a wrapper for
// (Duration, Timeout)

/// A ScheduledTimeout is a tuple of a `Duration` (time) and `Timeout` (handler)
struct ScheduledTimeout(Duration, Timeout);

// Implement the Debug trait so we can log it
impl fmt::Debug for ScheduledTimeout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScheduledTimeout: {:?}", self.0)
    }
}

// The following traits are implemented so that we can achieve the "min" ordering

impl Eq for ScheduledTimeout {}

impl PartialEq for ScheduledTimeout {
    fn eq(&self, other: &ScheduledTimeout) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for ScheduledTimeout {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for ScheduledTimeout {
    fn cmp(&self, other: &ScheduledTimeout) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord,
        }
    }
}

/// Our TimeoutQueue. In reality it's just a thin wrapper around a BinaryHeap, but we wrap it so
/// we can have a min-heap without having to worry about the ScheduledTimeout structure elsewhere in
/// our code
pub struct TimeoutQueue {
    heap: BinaryHeap<ScheduledTimeout>,
}

impl TimeoutQueue {
    pub fn new() -> TimeoutQueue {
        TimeoutQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn peek(&self) -> Option<Duration> {
        self.heap.peek().map(|&ScheduledTimeout(time, _)| time)
    }

    pub fn push(&mut self, time: Duration, event: Timeout) {
        self.heap.push(ScheduledTimeout(time, event));
    }

    pub fn pop(&mut self) -> Option<Timeout> {
        self.heap.pop().map(|ScheduledTimeout(_, event)| event)
    }
}
