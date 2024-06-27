#![allow(clippy::collapsible_if)]

use embassy_time::Instant;

use crate::config::{AUTO_MOUSE_DURATION, AUTO_MOUSE_THRESHOLD};

pub struct Aml {
    start: Option<Instant>,
    move_acc: u8,
}

impl Aml {
    pub fn new() -> Self {
        Self {
            start: None,
            move_acc: 0,
        }
    }

    pub fn enabled(&mut self, now: Instant, mouse_event: (i8, i8), continue_aml: bool) -> bool {
        if let Some(start) = self.start {
            if mouse_event != (0, 0) || continue_aml {
                self.start = Some(now);
            } else if now.duration_since(start) > AUTO_MOUSE_DURATION {
                self.start = None;
                self.move_acc = 0;
            }
        } else {
            if mouse_event == (0, 0) {
                self.move_acc = 0;
            } else {
                self.move_acc += mouse_event.0.unsigned_abs() + mouse_event.1.unsigned_abs();
            }

            if self.move_acc > AUTO_MOUSE_THRESHOLD {
                self.start = Some(now);
                self.move_acc = 0;
            }
        }

        self.start.is_some()
    }
}
