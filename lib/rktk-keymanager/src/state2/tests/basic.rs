use super::prelude::*;
use pretty_assertions::assert_eq;

#[test]
pub fn first_empty_second_none() {
    let mut state = new_state(EMPTY_KEYMAP);

    let report = update!(state, time(0));
    assert_eq!(report, NONE_REPORT, "Nothing happens");

    let report = update!(state, time(0));
    assert_eq!(report, NONE_REPORT, "Nothing happens, second");
}

#[test]
pub fn key_press_release() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].keymap[0][0] = KeyAction::Normal(KeyCode::Key(Key::A));
    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    assert_eq!(
        report,
        report_with_keycodes([0x04, 0, 0, 0, 0, 0]),
        "Key 'a' pressed"
    );

    let report = update!(state, time(0), (0, 0, false));
    assert_eq!(report, KEYBOARD_ONLY_REPORT, "Key 'a' released");
}
