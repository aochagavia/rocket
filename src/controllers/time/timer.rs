use std::time::Duration;

/// A `Timer` is used to trigger events in specified intervals
///
/// Each time the `update` function is called, the timer will check whether
/// the interval has elapsed. If that is the case, the provided action will
/// be triggered. Otherwise, it will be ignored.
pub struct Timer {
    last_triggered: Duration,
    interval: Duration,
}

impl Timer {
    pub fn new(interval: Duration) -> Timer {
        Timer {
            last_triggered: Duration::from_secs(0),
            interval,
        }
    }

    pub fn from_seconds(seconds: f32) -> Timer {
        Timer::new(Duration::from_millis((seconds * 1000.0) as u64))
    }

    pub fn update<F>(&mut self, current_time: Duration, mut action: F)
    where
        F: FnMut(),
    {
        if current_time - self.last_triggered > self.interval {
            self.last_triggered = current_time;
            action();
        }
    }
}
