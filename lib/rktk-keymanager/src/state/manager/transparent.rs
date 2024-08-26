//! Handles `transparent` report.
//!
//! `transparent` keys is type of keys that does not handled by keymanager and directly passed to host.
//! `crate::keycode::special::Special::FlashClear` is one of example.

use crate::{
    keycode::{special::Special, KeyCode},
    state::key_resolver::EventType,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransparentReport {
    pub flash_clear: bool,
}

impl Default for TransparentReport {
    fn default() -> Self {
        Self::new()
    }
}

impl TransparentReport {
    pub const fn new() -> Self {
        Self { flash_clear: false }
    }
}

pub struct TransparentLocalState {
    report: TransparentReport,
}

impl TransparentLocalState {
    pub const fn new() -> Self {
        Self {
            report: TransparentReport::new(),
        }
    }

    pub fn process_event(&mut self, kc: &KeyCode, event: EventType) {
        match (event, kc) {
            (EventType::Pressed, KeyCode::Special(Special::FlashClear)) => {
                self.report.flash_clear = true;
            }
            _ => {}
        };
    }

    pub fn report(self) -> TransparentReport {
        self.report
    }
}
