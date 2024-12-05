use core::time::Duration;

use usbd_hid::descriptor::MouseReport;

use crate::{
    keycode::{key::Key, special::Special, KeyCode},
    state::{
        action_handler::EventType,
        common::{CommonLocalState, CommonState},
        config::MouseConfig,
    },
};

use self::aml::Aml;

mod aml;
mod reporter;

/// Global mouse state
pub struct MouseState {
    scroll_mode: bool,
    reporter: reporter::MouseReportGenerator,
    aml: Aml,
    arrowball_move: (i8, i8),
    auto_mouse_layer: usize,
}

impl MouseState {
    pub fn new(config: MouseConfig) -> Self {
        Self {
            aml: Aml::new(
                Duration::from_millis(config.auto_mouse_duration as u64),
                config.auto_mouse_threshold,
            ),
            scroll_mode: false,
            reporter: reporter::MouseReportGenerator::new(
                config.scroll_divider_x,
                config.scroll_divider_y,
            ),
            arrowball_move: (0, 0),
            auto_mouse_layer: config.auto_mouse_layer as usize,
        }
    }
}

/// Loop-local mouse state
pub struct MouseLocalState {
    pub mouse_event: (i8, i8),
    pub mouse_button: u8,
    pub disable_aml: bool,
}

impl MouseLocalState {
    pub fn new(mouse_event: (i8, i8)) -> Self {
        Self {
            mouse_event,
            mouse_button: 0,
            disable_aml: false,
        }
    }

    pub fn process_event(
        &mut self,
        global_mouse_state: &mut MouseState,
        kc: &KeyCode,
        event: EventType,
    ) {
        match (kc, event) {
            (KeyCode::Mouse(btn), _) => self.mouse_button |= btn.bits(),
            (KeyCode::Special(Special::MoScrl), EventType::Released) => {
                global_mouse_state.scroll_mode = false;
            }
            (KeyCode::Special(Special::MoScrl), EventType::Pressed) => {
                global_mouse_state.scroll_mode = true;
            }
            (KeyCode::Special(Special::AmlReset), EventType::Pressed) => {
                self.disable_aml = true;
            }
            (_, EventType::Pressed) => {
                self.disable_aml = true;
            }
            _ => {}
        }
    }

    pub fn loop_end<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
    >(
        &mut self,
        common_state: &mut CommonState<LAYER, ROW, COL, ENCODER_COUNT>,
        common_local_state: &mut CommonLocalState,
        global_mouse_state: &mut MouseState,
        highest_layer: usize,
    ) {
        if common_state.keymap.layers[highest_layer].arrowmouse {
            global_mouse_state.arrowball_move.0 += self.mouse_event.0;
            global_mouse_state.arrowball_move.1 += self.mouse_event.1;

            let mut reset = true;
            if global_mouse_state.arrowball_move.1 > 50 {
                let _ = common_local_state.keycodes.insert(Key::Right as u8);
            } else if global_mouse_state.arrowball_move.1 < -50 {
                let _ = common_local_state.keycodes.insert(Key::Left as u8);
            } else if global_mouse_state.arrowball_move.0 > 50 {
                let _ = common_local_state.keycodes.insert(Key::Down as u8);
            } else if global_mouse_state.arrowball_move.0 < -50 {
                let _ = common_local_state.keycodes.insert(Key::Up as u8);
            } else {
                reset = false;
            }

            if reset {
                global_mouse_state.arrowball_move = (0, 0);
            }

            self.mouse_event = (0, 0);
        } else {
            global_mouse_state.arrowball_move = (0, 0);
            let (enabled, changed) = global_mouse_state.aml.enabled_changed(
                common_local_state.now,
                self.mouse_event,
                self.mouse_button != 0 || global_mouse_state.scroll_mode,
                self.disable_aml,
            );
            if changed {
                common_state.layer_active[global_mouse_state.auto_mouse_layer] = enabled;
            }
        }
    }

    pub fn report(self, global_state: &mut MouseState) -> Option<MouseReport> {
        global_state.reporter.gen(
            self.mouse_event,
            self.mouse_button,
            global_state.scroll_mode,
        )
    }
}
