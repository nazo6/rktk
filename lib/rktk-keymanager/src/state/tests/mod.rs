mod action;
mod basic;
mod keycode;
mod keymap;

#[allow(unused_imports)]
mod prelude {
    pub use crate::time::{Duration, Instant};
    pub use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

    pub(super) use super::super::{KeyChangeEvent, State, StateReport};
    pub(super) use super::keymap::EMPTY_KEYMAP;
    use crate::state::config::{
        KeyResolverConfig, MouseConfig, TapDanceConfig, MAX_TAP_DANCE_REPEAT_COUNT,
    };
    pub(super) use crate::{
        keycode::*,
        keycode::{key::*, layer::*, media::*, modifier::*, mouse::*, special::*, utils::*},
    };

    pub const fn time(ms: u64) -> Duration {
        Duration::from_millis(ms)
    }

    pub const ROWS: usize = 5;
    pub const COLS: usize = 14;
    pub const LAYER_COUNT: usize = 5;

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
                KeyChangeEvent {
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
                (0, 0),
                $now
)
        };
        ($state:expr, $now:expr) => {
            $state.update(
                &mut [],
                (0, 0),
                $now
)
        };
    }

    pub fn new_state(
        keymap: [crate::Layer<ROWS, COLS>; LAYER_COUNT],
    ) -> State<LAYER_COUNT, ROWS, COLS> {
        let mut tap_dance = [const { None }; MAX_TAP_DANCE_REPEAT_COUNT as usize];
        tap_dance[0] = Some(TapDanceConfig {
            tap: [
                Some(KeyCode::Key(Key::A)),
                Some(KeyCode::Key(Key::B)),
                Some(KeyCode::Layer(LayerOp::Toggle(2))),
                None,
            ],
            hold: [
                Some(KeyCode::Modifier(Modifier::LCtrl)),
                Some(KeyCode::Layer(LayerOp::Momentary(1))),
                None,
                None,
            ],
        });

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
                    tap_threshold: 500,
                    tap_dance_threshold: 100,
                    tap_dance,
                },
            },
        )
    }

    pub(crate) use key_change;
    pub(crate) use update;
}
