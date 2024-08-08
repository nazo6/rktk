#![allow(clippy::collapsible_if)]

use embassy_time::{Duration, Instant};

pub struct Aml {
    start: Option<Instant>,
    move_acc: u8,
    auto_mouse_duration: Duration,
    auto_mouse_threshold: u8,
}

impl Aml {
    pub fn new(auto_mouse_duration: Duration, auto_mouse_threshold: u8) -> Self {
        Self {
            start: None,
            move_acc: 0,
            auto_mouse_duration,
            auto_mouse_threshold,
        }
    }

    pub fn enabled_changed(
        &mut self,
        now: Instant,
        mouse_event: (i8, i8),
        continue_aml: bool,
    ) -> (bool, bool) {
        let mut changed = false;
        if let Some(start) = self.start {
            if mouse_event != (0, 0) || continue_aml {
                self.start = Some(now);
            } else if now.duration_since(start) > self.auto_mouse_duration {
                changed = true;
                self.start = None;
                self.move_acc = 0;
            }
        } else {
            if mouse_event == (0, 0) {
                self.move_acc = 0;
            } else {
                self.move_acc += mouse_event.0.unsigned_abs() + mouse_event.1.unsigned_abs();
            }

            if self.move_acc > self.auto_mouse_threshold {
                changed = true;
                self.start = Some(now);
                self.move_acc = 0;
            }
        }

        (self.start.is_some(), changed)
    }
}