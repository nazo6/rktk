//! Debounce driver implementations.

use rktk::{
    config::constant::CONFIG,
    drivers::interface::debounce::{DebounceDriver, KeyChangeEvent},
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
    deboune_only_pressed: bool,
}

impl EagerDebounceDriver {
    /// Create a new `EagerDebounceDriver` instance.
    ///
    /// # Arguments
    /// * debounce_time - The debounce time.
    /// * deboune_only_pressed - If true, only debounce pressed events.
    pub fn new(debounce_time: embassy_time::Duration, deboune_only_pressed: bool) -> Self {
        Self {
            last: [[None; CONFIG.keyboard.cols as usize]; CONFIG.keyboard.rows as usize],
            debounce_time,
            deboune_only_pressed,
        }
    }
}

impl DebounceDriver for EagerDebounceDriver {
    fn should_ignore_event(&mut self, event: &KeyChangeEvent, now: embassy_time::Instant) -> bool {
        let last = self.last[event.row as usize][event.col as usize];
        if let Some(last) = last {
            if now - last < self.debounce_time {
                if !self.deboune_only_pressed {
                    return event.pressed;
                } else {
                    return true;
                }
            }
        }
        self.last[event.row as usize][event.col as usize] = Some(now);
        false
    }
}
