use usbd_hid::descriptor::KeyboardReport;

use crate::{
    keycode::KeyCode,
    state::{common::CommonLocalState, key_resolver::EventType},
};

mod reporter;

pub struct KeyboardState {
    reporter: reporter::KeyboardReportGenerator,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            reporter: reporter::KeyboardReportGenerator::new(),
        }
    }
}

pub struct KeyboardLocalState {
    pub modifier: u8,
}

impl KeyboardLocalState {
    pub fn new() -> Self {
        Self { modifier: 0 }
    }

    pub fn process_event(
        &mut self,
        common_local_state: &mut CommonLocalState,
        kc: &KeyCode,
        event: EventType,
    ) {
        match kc {
            KeyCode::Key(key) => {
                if let EventType::Pressed = event {
                    common_local_state.normal_key_pressed = true;
                }
                common_local_state.keycodes.push(*key as u8).ok();
            }
            KeyCode::Modifier(mod_key) => {
                self.modifier |= mod_key.bits();
            }
            _ => {}
        };
    }

    pub fn report(
        self,
        common_local_state: &CommonLocalState,
        global_state: &mut KeyboardState,
    ) -> Option<KeyboardReport> {
        global_state
            .reporter
            .gen(&common_local_state.keycodes, self.modifier)
    }
}
