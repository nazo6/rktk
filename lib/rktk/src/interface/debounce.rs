use rktk_keymanager::state::KeyChangeEvent;

use crate::config::static_config::CONFIG;

/// `debounce` is way to reduce chatter or noise.
pub trait DebounceDriver {
    fn should_ignore_event(&mut self, event: &KeyChangeEvent, now: embassy_time::Instant) -> bool;
}

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

pub enum DummyDebounceDriver {}
impl DebounceDriver for DummyDebounceDriver {
    fn should_ignore_event(&mut self, _: &KeyChangeEvent, _: embassy_time::Instant) -> bool {
        unreachable!()
    }
}
