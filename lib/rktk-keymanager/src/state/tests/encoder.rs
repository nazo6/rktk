use crate::{keymap::Keymap, state::EncoderDirection};

use super::prelude::*;
use pretty_assertions::assert_eq;

const ENCODER_KEYMAP: Keymap<LAYER_COUNT, ROWS, COLS, ENC_COUNT> = const {
    let mut keymap = EMPTY_KEYMAP;
    keymap.encoder_keys[0] = (KeyCode::Key(Key::B), KeyCode::Key(Key::A));
    keymap
};

#[test]
pub fn encoder_clockwise() {
    let mut state = new_state(ENCODER_KEYMAP);

    let _ = update!(state, time(0));

    let report = state.update(Event::Encoder((0, EncoderDirection::Clockwise)), time(0));

    let mut expected = KEYBOARD_ONLY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes = [0x04, 0, 0, 0, 0, 0];
    assert_eq!(report, expected, "In first report, key `A` should be sent");

    let report = update!(state, time(0));
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
        Event::Encoder((0, EncoderDirection::CounterClockwise)),
        time(0),
    );

    let mut expected = KEYBOARD_ONLY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes = [0x05, 0, 0, 0, 0, 0];
    assert_eq!(report, expected, "In first report, key `B` should be sent");

    let report = update!(state, time(0));
    assert_eq!(
        report, KEYBOARD_ONLY_REPORT,
        "In second send, report should not be sent"
    );
}
