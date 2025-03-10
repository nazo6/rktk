mod action;
mod basic;
mod combo;
mod encoder;
mod keycode;
mod keymap;
mod mouse;

#[allow(unused_imports)]
mod prelude {
    use core::time::Duration;

    pub(super) use super::keymap::EMPTY_KEYMAP;
    use crate::interface::state::config::{
        ComboConfig, KeyResolverConfig, MouseConfig, StateConfig, TapDanceConfig, TapHoldConfig,
    };
    pub(super) use crate::{
        interface::state::input_event::InputEvent,
        keycode::{key::*, layer::*, media::*, modifier::*, mouse::*, special::*, utils::*, *},
        time::Instant,
    };
    pub use crate::{
        interface::state::input_event::KeyChangeEvent,
        keymap::{Keymap, TapDanceDefinition},
        state::{
            hid_report::{HidReportState, Report},
            hooks::EmptyHooks,
            State,
        },
    };
    pub use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

    pub const fn time(ms: u32) -> Duration {
        Duration::from_millis(ms as u64)
    }

    pub const ROWS: usize = 5;
    pub const COLS: usize = 14;
    pub const LAYER_COUNT: usize = 5;
    pub const ENC_COUNT: usize = 1;

    /// All report is None. This means there is no report to send.
    pub const NONE_REPORT: Report = Report {
        keyboard_report: None,
        mouse_report: None,
        media_keyboard_report: None,
        highest_layer: 0,
    };
    pub const KEYBOARD_ONLY_REPORT: Report = Report {
        keyboard_report: Some(KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0, 0, 0, 0, 0, 0],
        }),
        mouse_report: None,
        media_keyboard_report: None,
        highest_layer: 0,
    };
    pub const MOUSE_ONLY_REPORT: Report = Report {
        keyboard_report: None,
        mouse_report: Some(MouseReport {
            buttons: 0,
            x: 0,
            y: 0,
            wheel: 0,
            pan: 0,
        }),
        media_keyboard_report: None,
        highest_layer: 0,
    };

    pub const fn report_with_keycodes(keycodes: [u8; 6]) -> Report {
        let mut report = KEYBOARD_ONLY_REPORT;
        report.keyboard_report.as_mut().unwrap().keycodes = keycodes;
        report
    }

    macro_rules! update {
        ($state:expr, $now:expr, ($row:expr, $col:expr, $pressed:expr)) => {
            $state.update(
                InputEvent::Key(KeyChangeEvent {
                    row: $row,
                    col: $col,
                    pressed: $pressed,
                }),
                $now,
            )
        };
        ($state:expr, $now:expr) => {
            $state.update(InputEvent::None, $now)
        };
    }

    pub fn new_state(
        keymap: Keymap<LAYER_COUNT, ROWS, COLS, ENC_COUNT, 2, 4, 2, 3>,
    ) -> HidReportState<EmptyHooks, LAYER_COUNT, ROWS, COLS, ENC_COUNT, 5, 2, 4, 2, 3> {
        HidReportState::new(
            keymap,
            StateConfig {
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
                    combo: ComboConfig { threshold: 20 },
                },
            },
            EmptyHooks,
        )
    }

    pub(crate) use update;
}
