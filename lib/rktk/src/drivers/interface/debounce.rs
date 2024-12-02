//! Debounce driver type
//!
//! `debounce` is way to reduce chatter or noise this can be achieved by ignoring events that are too close to each other in time.

use rktk_keymanager::state::KeyChangeEvent;

use crate::config::static_config::CONFIG;

/// Debounce driver interface
pub trait DebounceDriver {
    /// Determines whether events occurring at a certain time should be ignored.
    fn should_ignore_event(&mut self, event: &KeyChangeEvent, now: embassy_time::Instant) -> bool;
}
