mod action;
mod basic;
mod keycode;
mod keymap;

#[allow(unused_imports)]
mod prelude {

    pub use embassy_time::{Duration, Instant};
    pub use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

    pub(super) use super::super::{State, StateReport};
    pub(super) use super::keymap::EMPTY_KEYMAP;
    pub(super) use crate::{
        interface::keyscan::{Hand, KeyChangeEventOneHand},
        keycode::*,
        keycode::{key::*, layer::*, media::*, modifier::*, mouse::*, special::*, utils::*},
    };

    pub const fn time(ms: u64) -> Instant {
        Instant::from_millis(ms)
    }

    pub const NONE_REPORT: StateReport = StateReport {
        keyboard_report: None,
        mouse_report: None,
        media_keyboard_report: None,
        highest_layer: 0,
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
        highest_layer: 0,
    };

    macro_rules! key_change {
        ($(($row:expr, $col:expr, $pressed:expr)),*) => {
            [$(
                KeyChangeEventOneHand {
                    row: $row,
                    col: $col,
                    pressed: $pressed,
                }
            ),*]
        };
    }

    macro_rules! update {
        ($state:expr, $now:expr, $($arg:tt),+) => {
            $state.update(
                &mut key_change!($($arg),*),
                &mut [],
                (0, 0),
                $now
)
        };
        ($state:expr, $now:expr) => {
            $state.update(
                &mut [],
                &mut [],
                (0, 0),
                $now
)
        };
    }

    pub(crate) use key_change;
    pub(crate) use update;
}
