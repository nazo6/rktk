use usbd_hid::descriptor::KeyboardReport;

use crate::{
    keycode::KeyCode,
    state::{
        common::{CommonLocalState, CommonState},
        pressed::{KeyStatus, KeyStatusEvent},
    },
};

use super::interface::LocalStateManager;

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
    pub media_key: Option<u16>,
}

impl KeyboardLocalState {
    pub fn new() -> Self {
        Self {
            modifier: 0,
            media_key: None,
        }
    }
}

impl LocalStateManager for KeyboardLocalState {
    type GlobalState = KeyboardState;

    type Report = KeyboardReport;

    fn process_event(
        &mut self,
        _common_state: &mut CommonState,
        common_local_state: &mut CommonLocalState,
        _global_state: &mut Self::GlobalState,
        kc: &KeyCode,
        event: &KeyStatusEvent,
    ) {
        match kc {
            KeyCode::Key(key) => {
                if let KeyStatus::Pressed = event.change_type {
                    common_local_state.normal_key_pressed = true;
                }
                common_local_state.keycodes.push(*key as u8).ok();
            }
            KeyCode::Media(key) => {
                self.media_key = Some(*key as u16);
            }
            KeyCode::Modifier(mod_key) => {
                self.modifier |= mod_key.bits();
            }
            _ => {}
        };
    }

    fn report(
        self,
        _common_state: &CommonState,
        common_local_state: &CommonLocalState,
        global_state: &mut Self::GlobalState,
    ) -> Option<Self::Report> {
        global_state
            .reporter
            .gen(&common_local_state.keycodes, self.modifier)
    }
}
