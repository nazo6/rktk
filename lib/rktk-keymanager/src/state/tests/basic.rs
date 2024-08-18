use super::prelude::*;
use pretty_assertions::assert_eq;

#[test]
pub fn first_empty_second_none() {
    let mut state = new_state(EMPTY_KEYMAP);

    let report = update!(state, time(0));
    assert_eq!(
        report, EMPTY_REPORT,
        "In first report, empty report should be sent"
    );

    let report = update!(state, time(0));
    assert_eq!(
        report, NONE_REPORT,
        "In second send, empty report should not be sent"
    );
}

#[test]
pub fn key_press_release() {
    let mut keymap = EMPTY_KEYMAP;
    keymap[0].map[0][0] = KeyAction::Normal(KeyCode::Key(Key::A));
    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    let mut expected = KEYBOARD_ONLY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes = [0x04, 0, 0, 0, 0, 0];
    assert_eq!(report, expected, "Key 'a' pressed");

    let report = update!(state, time(0), (0, 0, false));
    assert_eq!(report, KEYBOARD_ONLY_REPORT, "Key 'a' released");
}
