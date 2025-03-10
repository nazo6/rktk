use crate::{
    interface::state::{
        config::MouseConfig,
        output_event::{EventType, OutputEvent},
    },
    keycode::{key::Key, special::Special, KeyCode},
    time::Duration,
};

use self::aml::Aml;

mod aml;
mod reporter;

/// Global mouse state
pub struct MouseState {
    scroll_mode: bool,
    aml: Aml,
    arrow_mouse_move: (i8, i8),
    auto_mouse_layer: usize,
}

impl MouseState {
    pub fn new(config: MouseConfig) -> Self {
        Self {
            aml: Aml::new(
                Duration::from_millis(config.auto_mouse_duration),
                config.auto_mouse_threshold,
            ),
            scroll_mode: false,
            arrow_mouse_move: (0, 0),
            auto_mouse_layer: config.auto_mouse_layer as usize,
        }
    }

    pub fn start_update<'a>(&'a mut self) -> MouseUpdater<'a> {
        MouseUpdater {
            state: self,
            mouse_move: (0, 0),
            disable_aml: false,
            extend_aml: false,
        }
    }
}

/// Loop-local mouse state
pub struct MouseUpdater<'a> {
    state: &'a mut MouseState,
    mouse_move: (i8, i8),
    disable_aml: bool,
    extend_aml: bool,
}

impl<'a> MouseUpdater<'a> {
    pub fn update_by_keycode(
        &mut self,
        kc: &KeyCode,
        event: EventType,
        mut cb: impl FnMut(OutputEvent),
    ) {
        match (kc, event) {
            (KeyCode::Mouse(m), et) => {
                cb(OutputEvent::MouseButton((*m, et)));
                if et != EventType::Released {
                    self.extend_aml = true;
                }
            }
            (KeyCode::Special(Special::MoScrl), EventType::Released) => {
                self.state.scroll_mode = false;
            }
            (KeyCode::Special(Special::MoScrl), EventType::Pressed) => {
                self.state.scroll_mode = true;
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

    pub fn update_by_mouse_move(&mut self, (x, y): (i8, i8)) {
        self.mouse_move.0 += x;
        self.mouse_move.1 += y;
    }

    pub fn end<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
        const TAP_DANCE_MAX_DEFINITIONS: usize,
        const TAP_DANCE_MAX_REPEATS: usize,
        const COMBO_KEY_MAX_DEFINITIONS: usize,
        const COMBO_KEY_MAX_SOURCES: usize,
    >(
        mut self,
        highest_layer: usize,
        shared_state: &mut super::super::shared::SharedState<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
        mut cb: impl FnMut(OutputEvent),
    ) {
        if shared_state.keymap.layers[highest_layer].arrow_mouse {
            self.state.arrow_mouse_move.0 += self.mouse_move.0;
            self.state.arrow_mouse_move.1 += self.mouse_move.1;

            let key = if self.state.arrow_mouse_move.1 > 50 {
                Some(Key::Right)
            } else if self.state.arrow_mouse_move.1 < -50 {
                Some(Key::Left)
            } else if self.state.arrow_mouse_move.0 > 50 {
                Some(Key::Down)
            } else if self.state.arrow_mouse_move.0 < -50 {
                Some(Key::Up)
            } else {
                None
            };

            if let Some(key) = key {
                cb(OutputEvent::Key((key, EventType::Pressed)));
                self.state.arrow_mouse_move = (0, 0);
            }

            self.mouse_move = (0, 0);
        } else {
            self.state.arrow_mouse_move = (0, 0);
            let (enabled, changed) = self.state.aml.enabled_changed(
                shared_state.now,
                self.mouse_move,
                self.extend_aml || self.state.scroll_mode,
                self.disable_aml,
            );
            if changed {
                shared_state.layer_active[self.state.auto_mouse_layer] = enabled;
            }
        }

        if self.mouse_move != (0, 0) {
            cb(OutputEvent::MouseMove(self.mouse_move));
        }
    }
}
