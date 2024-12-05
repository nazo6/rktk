use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::keycode::{key::Key, special::Special, KeyCode};
use crate::state::config::Output;

use super::action_handler::EventType;

mod keyboard;
pub mod reports;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransparentReport {
    pub flash_clear: bool,
    pub ble_bond_clear: bool,
    pub output: Output,
    pub bootloader: bool,
}

pub struct KeyCodeHandler {
    empty_kb_sent: bool,
}

impl KeyCodeHandler {
    pub fn start_process(&mut self) -> KeyCodeHandlerTransaction {
        KeyCodeHandlerTransaction {
            handler: self,
            reports: Reports::default(),
        }
    }
}

pub struct KeyCodeHandlerTransaction<'a> {
    handler: &'a mut KeyCodeHandler,
    reports: Reports,
}

impl<'a> KeyCodeHandlerTransaction<'a> {
    fn push_keycode(&mut self, key: Key) {
        if self.reports.keyboard_report.is_none() {
            self.reports.keyboard_report = Some(KeyboardReport::default());
        }
        let buf = &mut self.reports.keyboard_report.as_mut().unwrap().keycodes;

        for i in 0..buf.len() {
            if buf[i] == 0 {
                buf[i] = key as u8;
                break;
            }
        }
    }

    fn add_modifier(&mut self, modifier: crate::keycode::modifier::Modifier) {
        if self.reports.keyboard_report.is_none() {
            self.reports.keyboard_report = Some(KeyboardReport::default());
        }
        self.reports.keyboard_report.as_mut().unwrap().modifier |= modifier.bits();
    }

    pub fn process_keycode(&mut self, event: EventType, kc: KeyCode) {
        match (event, kc) {
            (_, KeyCode::Key(key)) => {
                self.push_keycode(key);
            }
            (_, KeyCode::Modifier(mod_key)) => {
                self.add_modifier(mod_key);
            }
            (EventType::Pressed, KeyCode::Special(Special::FlashClear)) => {
                self.reports.transparent_report.flash_clear = true;
            }
            (EventType::Pressed, KeyCode::Special(Special::OutputBle)) => {
                self.reports.transparent_report.output = Output::Ble;
            }
            (EventType::Pressed, KeyCode::Special(Special::OutputUsb)) => {
                self.reports.transparent_report.output = Output::Usb;
            }
            (EventType::Pressed, KeyCode::Special(Special::BleBondClear)) => {
                self.reports.transparent_report.ble_bond_clear = true;
            }
            (EventType::Pressed, KeyCode::Special(Special::Bootloader)) => {
                self.reports.transparent_report.bootloader = true;
            }
            _ => {}
        };
    }
    pub fn process_mouse(movement: (i8, i8)) {}
    pub fn generate_report(self) -> Reports {
        self.reports
    }
}
