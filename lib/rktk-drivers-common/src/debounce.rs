//! Debounce driver implementations.

use rktk::{
    config::constant::CONST_CONFIG,
    drivers::interface::debounce::{DebounceDriver, KeyChangeEvent},
};

/// Debounce driver that implements `Eager debouncing`
///
/// Debounces key using "eager debouncing" strategy.
/// Ignores all events related to a key for a certain period of time after that event is reported.
/// ref: [Debouncing | ZMK](https://zmk.dev/docs/features/debouncing#eager-debouncing)
pub struct EagerDebounceDriver {
    last: [[Option<embassy_time::Instant>; CONST_CONFIG.keyboard.cols as usize];
        CONST_CONFIG.keyboard.rows as usize],
    debounce_time: embassy_time::Duration,
    deboune_only_pressed: bool,
}

impl EagerDebounceDriver {
    /// Create a new `EagerDebounceDriver` instance.
    ///
    /// # Arguments
    /// * debounce_time - The debounce time.
    /// * deboune_only_pressed - If true, only debounce pressed events.
    pub const fn new(debounce_time: embassy_time::Duration, deboune_only_pressed: bool) -> Self {
        Self {
            last: [[None; CONST_CONFIG.keyboard.cols as usize];
                CONST_CONFIG.keyboard.rows as usize],
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
                if self.deboune_only_pressed {
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

#[cfg(test)]
mod tests {
    use embassy_time::{Duration, Instant};
    use rktk::drivers::interface::{debounce::DebounceDriver as _, keyscan::KeyChangeEvent};

    use super::EagerDebounceDriver;

    #[test]
    fn eager_debouncer_only_pressed() {
        // PRESS    Accepted
        // 5ms
        // RELEASE  Maybe chatter but accepted
        // 3ms
        // PRESS    Ignored
        // ...
        // 92ms
        // ...
        // PRESS    Accepted

        let mut d = EagerDebounceDriver::new(Duration::from_millis(10), true);

        assert!(
            !d.should_ignore_event(
                &KeyChangeEvent {
                    row: 0,
                    col: 0,
                    pressed: true,
                },
                Instant::from_millis(0),
            ),
            "Key press event should not be ignored"
        );

        assert!(
            !d.should_ignore_event(
                &KeyChangeEvent {
                    row: 0,
                    col: 0,
                    pressed: false,
                },
                Instant::from_millis(5),
            ),
            "Key release event before debounce_time should not be ignored"
        );

        assert!(
            d.should_ignore_event(
                &KeyChangeEvent {
                    row: 0,
                    col: 0,
                    pressed: true,
                },
                Instant::from_millis(8),
            ),
            "Key press event before debounce_time should be ignored"
        );

        assert!(
            !d.should_ignore_event(
                &KeyChangeEvent {
                    row: 0,
                    col: 0,
                    pressed: true,
                },
                Instant::from_millis(100),
            ),
            "Key press event after debounce_time should not be ignored"
        );
    }
}
