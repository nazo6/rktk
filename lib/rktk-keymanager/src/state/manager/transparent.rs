//! Handles `transparent` report.
//!
//! `transparent` keys is type of keys that does not handled by keymanager and directly passed to host.
//! `crate::keycode::special::Special::FlashClear` is one of example.

use crate::{
    keycode::{special::Special, KeyCode},
    state::key_resolver::EventType,
};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct TransparentReport {
    pub flash_clear: bool,
}

pub struct TransparentLocalState {
    report: TransparentReport,
}

impl TransparentLocalState {
    pub fn new() -> Self {
        Self {
            report: TransparentReport::default(),
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
