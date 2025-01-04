//! Handles `transparent` report.
//!
//! `transparent` keys is type of keys that does not handled by keymanager and directly passed to host.
//! `crate::keycode::special::Special::FlashClear` is one of example.

use crate::{
    interface::{report::TransparentReport, Output},
    keycode::{special::Special, KeyCode},
    state::key_resolver::EventType,
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

    pub fn gen_report(&mut self, local_state: TransparentLocalState) -> TransparentReport {
        self.output = local_state.report.output;

        local_state.report
    }
}

pub struct TransparentLocalState {
    report: TransparentReport,
}

impl TransparentLocalState {
    pub const fn new(global_state: &TransparentState) -> Self {
        Self {
            report: TransparentReport {
                flash_clear: false,
                output: global_state.output,
                ble_bond_clear: false,
                bootloader: false,
                power_off: false,
            },
        }
    }

    pub fn process_event(&mut self, kc: &KeyCode, event: EventType) {
        macro_rules! pressed_enable {
            ($($kc:ident $field:ident: $val:expr),*) => {
                match (event, kc) {
                    $( (EventType::Pressed, KeyCode::Special(Special::$kc)) => {
                        self.report.$field = $val;
                    } )*
                    _ => {}
                };
            };
        }

        pressed_enable! {
            FlashClear flash_clear: true,
            BleBondClear ble_bond_clear: true,
            Bootloader bootloader: true,
            OutputBle output: Output::Ble,
            OutputUsb output: Output::Usb,
            PowerOff power_off: true

        }
    }

    pub fn report(self, global_state: &mut TransparentState) -> TransparentReport {
        global_state.gen_report(self)
    }
}
