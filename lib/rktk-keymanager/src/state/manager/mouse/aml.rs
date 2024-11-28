#![allow(clippy::collapsible_if)]

use crate::time::{Duration, Instant};

pub enum AmlState {
    /// Represents the active state of the AML.
    /// Contains u8 which is the mouse movement to track aml threshold.
    Inactive(u8),
    Active(Instant),
}

pub struct Aml {
    state: AmlState,
    auto_mouse_duration: Duration,
    auto_mouse_threshold: u8,
}

impl Aml {
    pub fn new(auto_mouse_duration: Duration, auto_mouse_threshold: u8) -> Self {
        Self {
            state: AmlState::Inactive(0),
            auto_mouse_duration,
            auto_mouse_threshold,
        }
    }

    pub fn enabled_changed(
        &mut self,
        now: Instant,
        mouse_event: (i8, i8),
        // If true, continues aml ignoring other conditions.
        // Typically used when mouse button is pressed.
        // This takes precedence over `force_disable_aml`.
        force_continue_aml: bool,
        // If true, disables aml ignoring other conditions.
        // Typically used when other key is pressed to acheive `HOLD_ON_OTHER_KEY_PRESS`.
        force_disable_aml: bool,
    ) -> (bool, bool) {
        let mut changed = false;

        match &mut self.state {
            AmlState::Active(start_time) => {
                if mouse_event != (0, 0) || force_continue_aml {
                    *start_time = now;
                } else if (now - *start_time) > self.auto_mouse_duration || force_disable_aml {
                    changed = true;
                    self.state = AmlState::Inactive(0);
                }
            }
            AmlState::Inactive(movement) => {
                *movement += mouse_event.0.unsigned_abs() + mouse_event.1.unsigned_abs();
                if *movement > self.auto_mouse_threshold {
                    changed = true;
                    self.state = AmlState::Active(now);
                }
            }
        }

        (matches!(self.state, AmlState::Active(_)), changed)
    }
}
