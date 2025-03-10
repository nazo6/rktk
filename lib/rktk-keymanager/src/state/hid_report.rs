use heapless::Vec;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::interface::state::{
    input_event::InputEvent,
    output_event::{EventType, OutputEvent},
    KeymapInfo,
};

use super::hooks::Hooks;

#[derive(Debug, PartialEq, Clone)]
pub struct Report {
    pub keyboard_report: Option<KeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub highest_layer: u8,
}

pub struct HidReportState<
    H: Hooks,
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const ONESHOT_STATE_SIZE: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
> {
    state: super::State<
        H,
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        ONESHOT_STATE_SIZE,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >,
    next_send_keyboard_report: bool,
}

impl<
        H: Hooks,
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
        const ONESHOT_STATE_SIZE: usize,
        const TAP_DANCE_MAX_DEFINITIONS: usize,
        const TAP_DANCE_MAX_REPEATS: usize,
        const COMBO_KEY_MAX_DEFINITIONS: usize,
        const COMBO_KEY_MAX_SOURCES: usize,
    >
    HidReportState<
        H,
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        ONESHOT_STATE_SIZE,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >
{
    pub fn new(
        keymap: crate::keymap::Keymap<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
        config: crate::interface::state::config::StateConfig,
        hooks: H,
    ) -> Self {
        Self {
            state: super::State::new(keymap, config, hooks),
            next_send_keyboard_report: false,
        }
    }

    pub fn update(&mut self, event: InputEvent, since_last_update: core::time::Duration) -> Report {
        self.update_with_cb(event, since_last_update, |_| {})
    }

    pub fn update_with_cb(
        &mut self,
        event: InputEvent,
        since_last_update: core::time::Duration,
        mut cb: impl FnMut(OutputEvent),
    ) -> Report {
        let mut keyboard_change = self.next_send_keyboard_report;
        let mut keys: Vec<u8, 6> = Vec::new();
        let mut modifier = 0u8;

        let mut mouse_change = false;
        let mut movement = (0, 0);
        let mut scroll = (0, 0);
        let mut mouse_buttons = 0u8;

        let mut media_keyboard_change = false;
        let mut media_keys = 0u16;

        self.state.update(event, since_last_update, |ev| {
            match ev {
                OutputEvent::Key((key, ev)) => {
                    if ev != EventType::Pressing {
                        keyboard_change = true;
                    }
                    if ev != EventType::Released && !keys.contains(&(key as u8)) {
                        let _ = keys.push(key as u8);
                    }
                    // If both `Pressing` and `Released` events are sent same time, that means key
                    // should be released in next report.
                    if ev == EventType::Released && keys.contains(&(key as u8)) {
                        self.next_send_keyboard_report = true;
                    }
                }
                OutputEvent::Modifier((m, ev)) => {
                    if ev != EventType::Pressing {
                        keyboard_change = true;
                    }
                    if ev != EventType::Released {
                        modifier |= m as u8;
                    }
                }
                OutputEvent::MouseButton((m, ev)) => {
                    if ev != EventType::Pressing {
                        mouse_change = true;
                    }
                    if ev != EventType::Released {
                        mouse_buttons |= m as u8;
                    }
                }
                OutputEvent::MediaKey((m, ev)) => {
                    if ev != EventType::Pressing {
                        media_keyboard_change = true;
                    }
                    media_keys = m as u16;
                }
                OutputEvent::MouseMove(m) => {
                    mouse_change = true;
                    movement.0 += m.0;
                    movement.1 += m.1;
                }
                OutputEvent::MouseScroll((pan, wheel)) => {
                    mouse_change = true;
                    scroll.0 += pan;
                    scroll.1 += wheel;
                }
                _ => {}
            }
            cb(ev);
        });

        Report {
            keyboard_report: if keyboard_change {
                keys.resize_default(6).unwrap();
                let keycodes = keys.into_array().unwrap();
                Some(KeyboardReport {
                    keycodes,
                    modifier,
                    leds: 0,
                    reserved: 0,
                })
            } else {
                None
            },
            mouse_report: if mouse_change {
                Some(MouseReport {
                    buttons: mouse_buttons,
                    x: movement.0,
                    y: movement.1,
                    pan: scroll.0,
                    wheel: scroll.1,
                })
            } else {
                None
            },
            media_keyboard_report: if media_keyboard_change {
                Some(MediaKeyboardReport {
                    usage_id: media_keys,
                })
            } else {
                None
            },
            highest_layer: self.state.shared.highest_layer() as u8,
        }
    }

    pub fn inner(
        &self,
    ) -> &super::State<
        H,
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        ONESHOT_STATE_SIZE,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    > {
        &self.state
    }

    pub fn get_keymap_info() -> KeymapInfo {
        KeymapInfo {
            layer_count: LAYER as u8,
            max_tap_dance_key_count: TAP_DANCE_MAX_DEFINITIONS as u8,
            max_tap_dance_repeat_count: TAP_DANCE_MAX_REPEATS as u8,
            oneshot_state_size: ONESHOT_STATE_SIZE as u8,
        }
    }

    pub fn reset_with_config(
        &mut self,
        keymap: crate::keymap::Keymap<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
        config: crate::interface::state::config::StateConfig,
    ) {
        self.next_send_keyboard_report = false;
        self.state.reset_with_config(keymap, config);
    }
}
