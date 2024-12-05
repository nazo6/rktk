use super::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn combo() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::Normal(KeyCode::Key(Key::G));
    keymap.layers[0].map[0][1] = KeyAction::Normal(KeyCode::Key(Key::H));

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    let expected = NONE_REPORT;
    assert_eq!(report, expected, "Key 'a' is pressed. Before combo timeout");

    let report = update!(state, time(20), (0, 1, true));
    let mut expected = KEYBOARD_ONLY_REPORT;
    // I key
    expected.keyboard_report.as_mut().unwrap().keycodes[0] = 0x0C;
    assert_eq!(
        report, expected,
        "Key 'b' is pressed before combo timeout. Combo key sent."
    );

    let report = update!(state, time(100));
    assert_eq!(report, expected, "Both key still pressed.");

    let report = update!(state, time(150), (0, 0, false));
    assert_eq!(report, expected, "Key 'A' released");

    let report = update!(state, time(200), (0, 1, false));
    let expected = KEYBOARD_ONLY_REPORT;
    assert_eq!(report, expected, "Key 'B' released. Stop sending");
}

#[test]
fn combo_sideeffect() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::Normal(KeyCode::Key(Key::G));

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    let expected = NONE_REPORT;
    assert_eq!(report, expected, "Key 'a' is pressed. Before combo timeout");

    let report = update!(state, time(100), (0, 0, true));
    let mut expected = KEYBOARD_ONLY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes[0] = 0x0A;
    assert_eq!(
        report, expected,
        "Key 'a' is pressed exceeding combo timeout. Key 'a' is sent."
    );

    let report = update!(state, time(100));
    assert_eq!(report, expected, "Time elapsed. Still 'a' is pressed.");

    let report = update!(state, time(200), (0, 0, false));
    let expected = KEYBOARD_ONLY_REPORT;
    assert_eq!(report, expected, "Key 'A' released. Stop sending");
}
