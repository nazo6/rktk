//! Debounce driver implementations.

use rktk::{
    config::static_config::CONFIG, drivers::interface::debounce::DebounceDriver,
    keymanager::state::KeyChangeEvent,
};

/// Debounce driver that implements `Eager debouncing`
///
/// Debounces key using "eager debouncing" strategy.
/// Ignores all events related to a key for a certain period of time after that event is reported.
/// ref: [Debouncing | ZMK](https://zmk.dev/docs/features/debouncing#eager-debouncing)
pub struct EagerDebounceDriver {
    last: [[Option<embassy_time::Instant>; CONFIG.keyboard.cols as usize];
        CONFIG.keyboard.rows as usize],
    debounce_time: embassy_time::Duration,
}

impl EagerDebounceDriver {
    pub fn new(debounce_time: embassy_time::Duration) -> Self {
        Self {
            last: [[None; CONFIG.keyboard.cols as usize]; CONFIG.keyboard.rows as usize],
            debounce_time,
        }
    }
}

impl DebounceDriver for EagerDebounceDriver {
    fn should_ignore_event(&mut self, event: &KeyChangeEvent, now: embassy_time::Instant) -> bool {
        let last = self.last[event.row as usize][event.col as usize];
        if let Some(last) = last {
            if now - last < self.debounce_time {
                return true;
            }
        }
        self.last[event.row as usize][event.col as usize] = Some(now);
        false
    }
}
