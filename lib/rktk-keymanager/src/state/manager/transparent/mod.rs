//! Handles `transparent` report.
//!
//! `transparent` keys is type of keys that does not handled by keymanager and directly passed to host.
//! `crate::keycode::special::Special::FlashClear` is one of example.

mod report;

pub use report::TransparentReport;

use crate::{
    keycode::{special::Special, KeyCode},
    state::{config::Output, key_resolver::EventType},
};

pub struct TransparentState {
    output: Output,
}

impl TransparentState {
    pub fn new(initial_output: Output) -> Self {
        Self {
            output: initial_output,
        }
    }

    pub fn gen_report(&self, local_state: &TransparentLocalState) -> TransparentReport {
        TransparentReport {
            flash_clear: local_state.flash_clear,
            output: self.output,
            ble_bond_clear: local_state.ble_bond_clear,
            bootloader: local_state.bootloader,
        }
    }
}

pub struct TransparentLocalState {
    flash_clear: bool,
    ble_bond_clear: bool,
    bootloader: bool,
}

impl TransparentLocalState {
    pub const fn new() -> Self {
        Self {
            flash_clear: false,
            ble_bond_clear: false,
            bootloader: false,
        }
    }

    pub fn process_event(
        &mut self,
        global_state: &mut TransparentState,
        kc: &KeyCode,
        event: EventType,
    ) {
        match (event, kc) {
            (EventType::Pressed, KeyCode::Special(Special::FlashClear)) => {
                self.flash_clear = true;
            }
            (EventType::Pressed, KeyCode::Special(Special::OutputBle)) => {
                global_state.output = Output::Ble;
            }
            (EventType::Pressed, KeyCode::Special(Special::OutputUsb)) => {
                global_state.output = Output::Usb;
            }
            (EventType::Pressed, KeyCode::Special(Special::BleBondClear)) => {
                self.ble_bond_clear = true;
            }
            (EventType::Pressed, KeyCode::Special(Special::Bootloader)) => {
                self.bootloader = true;
            }
            _ => {}
        };
    }

    pub fn report(self, global_state: &mut TransparentState) -> TransparentReport {
        global_state.gen_report(&self)
    }
}
