use usbd_hid::descriptor::KeyboardReport;

use crate::{keycode::KeyCode, state::key_resolver::EventType};

use super::SharedLocalManagerState;

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
        common_local_state: &mut SharedLocalManagerState,
        kc: &KeyCode,
        event: EventType,
    ) {
        match (event, kc) {
            (EventType::Released, _) => {}
            (_, KeyCode::Key(key)) => {
                if let EventType::Pressed = event {
                    common_local_state.normal_key_pressed = true;
                }
                let _ = common_local_state.keycodes.insert(*key as u8);
            }
            (_, KeyCode::Modifier(mod_key)) => {
                self.modifier |= *mod_key as u8;
            }
            _ => {}
        };
    }

    pub fn report(
        self,
        common_local_state: &SharedLocalManagerState,
        global_state: &mut KeyboardState,
    ) -> Option<KeyboardReport> {
        global_state
            .reporter
            .gen(&common_local_state.keycodes, self.modifier)
    }
}
