use heapless::Vec;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::{
    interface::state::{
        KeymapInfo,
        input_event::InputEvent,
        output_event::{EventType, OutputEvent},
    },
    keycode::KeyCode,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Report {
    pub keyboard_report: Option<KeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub highest_layer: u8,
}

pub struct HidReportState<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const NORMAL_MAX_PRESSED_KEYS: usize,
    const ONESHOT_STATE_SIZE: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
> {
    state: super::State<
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        NORMAL_MAX_PRESSED_KEYS,
        ONESHOT_STATE_SIZE,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >,
    next_send_keyboard_report: bool,
    next_send_mkb_report: bool,
}

impl<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const NORMAL_MAX_PRESSED_KEYS: usize,
    const ONESHOT_STATE_SIZE: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
>
    HidReportState<
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        NORMAL_MAX_PRESSED_KEYS,
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
    ) -> Self {
        Self {
            state: super::State::new(keymap, config),
            next_send_keyboard_report: false,
            next_send_mkb_report: false,
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
        self.next_send_keyboard_report = false;
        let mut keys: Vec<u8, 6> = Vec::new();
        let mut modifier = 0u8;

        let mut mouse_change = false;
        let mut movement = (0, 0);
        let mut scroll = (0, 0);
        let mut mouse_buttons = 0u8;

        let mut media_keyboard_change = self.next_send_mkb_report;
        let mut media_keys = 0u16;

        self.state.update(event, since_last_update, |ev| {
            match ev {
                OutputEvent::KeyCode((kc, ev)) => {
                    match kc {
                        KeyCode::Key(key) => {
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
                        KeyCode::Media(m) => {
                            if ev != EventType::Pressing {
                                media_keyboard_change = true;
                            }
                            if ev != EventType::Released {
                                media_keys = m as u16;
                            }
                            if ev == EventType::Released && media_keys == m as u16 {
                                self.next_send_mkb_report = true;
                            }
                        }
                        KeyCode::Modifier(m) => {
                            if ev != EventType::Pressing {
                                keyboard_change = true;
                            }
                            if ev != EventType::Released {
                                modifier |= m as u8;
                            }
                        }
                        KeyCode::Mouse(m) => {
                            if ev != EventType::Pressing {
                                mouse_change = true;
                            }
                            if ev != EventType::Released {
                                mouse_buttons |= m as u8;
                            }
                        }
                        // Other types of key codes will not appear as the HID report.
                        _ => {}
                    }
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
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        NORMAL_MAX_PRESSED_KEYS,
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
}
