use crate::{interface::state::input_event::EncoderDirection, keymap::Keymap};

use super::prelude::*;
use pretty_assertions::assert_eq;

const ENCODER_KEYMAP: Keymap<LAYER_COUNT, ROWS, COLS, ENC_COUNT, 2, 4, 2, 3> = const {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].encoder_keys[0] = (Some(KeyCode::Key(Key::B)), Some(KeyCode::Key(Key::A)));
    keymap
};

#[test]
pub fn encoder_clockwise() {
    let mut state = new_state(ENCODER_KEYMAP);

    let _ = update!(state, time(0));

    let report = state.update(
        InputEvent::Encoder((0, EncoderDirection::Clockwise)),
        time(0),
    );

    assert_eq!(
        report,
        report_with_keycodes([0x04, 0, 0, 0, 0, 0]),
        "In first report, key `A` should be sent"
    );

    let report = update!(state, time(10));
    assert_eq!(
        report, KEYBOARD_ONLY_REPORT,
        "In second send, empty should not be sent"
    );
}

#[test]
pub fn encoder_counterclockwise() {
    let mut state = new_state(ENCODER_KEYMAP);

    let _ = update!(state, time(0));

    let report = state.update(
        InputEvent::Encoder((0, EncoderDirection::CounterClockwise)),
        time(0),
    );

    assert_eq!(
        report,
        report_with_keycodes([0x05, 0, 0, 0, 0, 0]),
        "In first report, key `B` should be sent"
    );

    let report = update!(state, time(0));
    assert_eq!(
        report, KEYBOARD_ONLY_REPORT,
        "In second send, empty report should be sent"
    );
}
