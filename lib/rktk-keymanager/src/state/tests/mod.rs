mod action;
mod basic;
mod encoder;
mod keycode;
mod keymap;
mod mouse;

#[allow(unused_imports)]
mod prelude {
    use core::time::Duration;

    pub(super) use super::super::{KeyChangeEvent, State, StateReport};
    pub(super) use super::keymap::EMPTY_KEYMAP;
    use crate::config::MAX_TAP_DANCE_REPEAT_COUNT;
    use crate::keymap::TapDanceDefinition;
    use crate::state::config::TapHoldConfig;
    pub(super) use crate::{
        keycode::{key::*, layer::*, media::*, modifier::*, mouse::*, special::*, utils::*, *},
        state::Event,
        state::TransparentReport,
        time::Instant,
    };
    use crate::{
        keymap::Keymap,
        state::config::{KeyResolverConfig, MouseConfig, Output, TapDanceConfig},
    };
    pub use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

    pub const fn time(ms: u32) -> Duration {
        Duration::from_millis(ms as u64)
    }

    pub const fn default_transparent_report() -> TransparentReport {
        TransparentReport {
            flash_clear: false,
            ble_bond_clear: false,
            output: Output::Usb,
            bootloader: false,
        }
    }

    pub const ROWS: usize = 5;
    pub const COLS: usize = 14;
    pub const LAYER_COUNT: usize = 5;
    pub const ENC_COUNT: usize = 1;

    pub const NONE_REPORT: StateReport = StateReport {
        keyboard_report: None,
        mouse_report: None,
        media_keyboard_report: None,
        highest_layer: 0,
        transparent_report: default_transparent_report(),
    };
    pub const EMPTY_REPORT: StateReport = StateReport {
        keyboard_report: Some(KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0, 0, 0, 0, 0, 0],
        }),
        mouse_report: Some(MouseReport {
            buttons: 0,
            x: 0,
            y: 0,
            wheel: 0,
            pan: 0,
        }),
        media_keyboard_report: Some(MediaKeyboardReport { usage_id: 0 }),
        transparent_report: default_transparent_report(),
        highest_layer: 0,
    };
    pub const KEYBOARD_ONLY_REPORT: StateReport = StateReport {
        keyboard_report: Some(KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0, 0, 0, 0, 0, 0],
        }),
        mouse_report: None,
        media_keyboard_report: None,
        transparent_report: default_transparent_report(),
        highest_layer: 0,
    };
    pub const MOUSE_ONLY_REPORT: StateReport = StateReport {
        keyboard_report: None,
        mouse_report: Some(MouseReport {
            buttons: 0,
            x: 0,
            y: 0,
            wheel: 0,
            pan: 0,
        }),
        media_keyboard_report: None,
        transparent_report: default_transparent_report(),
        highest_layer: 0,
    };

    macro_rules! update {
        ($state:expr, $now:expr, ($row:expr, $col:expr, $pressed:expr)) => {
            $state.update(
                Event::Key(KeyChangeEvent {
                    row: $row,
                    col: $col,
                    pressed: $pressed,
                }),
                $now,
            )
        };
        ($state:expr, $now:expr) => {
            $state.update(Event::None, $now)
        };
    }

    pub fn new_state(
        keymap: Keymap<LAYER_COUNT, ROWS, COLS, ENC_COUNT>,
    ) -> State<LAYER_COUNT, ROWS, COLS, ENC_COUNT> {
        State::new(
            keymap,
            crate::state::StateConfig {
                mouse: MouseConfig {
                    auto_mouse_layer: 1,
                    auto_mouse_duration: 500,
                    auto_mouse_threshold: 5,
                    scroll_divider_x: 20,
                    scroll_divider_y: -12,
                },
                key_resolver: KeyResolverConfig {
                    tap_hold: TapHoldConfig {
                        threshold: 300,
                        hold_on_other_key: true,
                    },
                    tap_dance: TapDanceConfig { threshold: 100 },
                },
                initial_output: Output::Usb,
            },
        )
    }

    pub(crate) use update;
}
