use crate::{
    interface::state::{
        config::MouseConfig,
        output_event::{EventType, OutputEvent},
    },
    keycode::{KeyCode, key::Key, special::Special},
    time::Duration,
};

use self::aml::Aml;

mod aml;

/// Global mouse state
pub struct MouseState {
    scroll_mode: bool,
    scroll_remained: (i8, i8),
    scroll_divider_x: i8,
    scroll_divider_y: i8,

    aml: Aml,
    arrow_mouse_move: (i8, i8),
    auto_mouse_layer: usize,
}

impl MouseState {
    pub fn new(config: MouseConfig) -> Self {
        Self {
            scroll_mode: false,
            scroll_remained: (0, 0),
            scroll_divider_x: config.scroll_divider_x,
            scroll_divider_y: config.scroll_divider_y,

            aml: Aml::new(
                Duration::from_millis(config.auto_mouse_duration),
                config.auto_mouse_threshold,
            ),
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
                cb(OutputEvent::KeyCode((KeyCode::Mouse(*m), et)));
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
                cb(OutputEvent::KeyCode((
                    KeyCode::Key(key),
                    EventType::Pressed,
                )));
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
            if self.state.scroll_mode {
                let pan_raw = self.mouse_move.0 + self.state.scroll_remained.0;
                let pan = pan_raw / self.state.scroll_divider_x;
                self.state.scroll_remained.0 = pan_raw % self.state.scroll_divider_x;

                let wheel_raw = self.mouse_move.1 + self.state.scroll_remained.1;
                let wheel = wheel_raw / self.state.scroll_divider_y;
                self.state.scroll_remained.1 = wheel_raw % self.state.scroll_divider_y;

                cb(OutputEvent::MouseScroll((pan, wheel)));
            } else {
                cb(OutputEvent::MouseMove(self.mouse_move));
            }
        }
    }
}
