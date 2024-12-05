use super::prelude::*;
use pretty_assertions::assert_eq;

mod tap_dance;

#[test]
fn normal_action() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::Normal(KeyCode::Key(Key::A));

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    let mut expected = KEYBOARD_ONLY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes = [0x04, 0, 0, 0, 0, 0];
    assert_eq!(report, expected, "Normal action key 'a' is pressed");
}

#[test]
fn normal2_action() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::Normal2(KeyCode::Key(Key::A), KeyCode::Key(Key::B));

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    let mut expected = KEYBOARD_ONLY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes = [0x04, 0x05, 0, 0, 0, 0];
    assert_eq!(
        report, expected,
        "Normal2 action key 'a' and 'b' are pressed"
    );
}

#[test]
fn taphold_action_hold() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::TapHold(KeyCode::Key(Key::A), KeyCode::Key(Key::B));

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    assert_eq!(report, NONE_REPORT, "TapHold action key, Just tapped");

    let report = update!(state, time(50));
    assert_eq!(
        report, NONE_REPORT,
        "TapHold action key, Sill in tapping term"
    );

    let report = update!(state, time(1000));
    let mut expected = NONE_REPORT;
    expected.keyboard_report = Some(KeyboardReport {
        keycodes: [0x05, 0, 0, 0, 0, 0],
        modifier: 0,
        reserved: 0,
        leds: 0,
    });
    assert_eq!(
        report, expected,
        "TapHold action key, tapping term exceeded, hold mode"
    );
}

#[test]
fn taphold_action_tap() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::TapHold(KeyCode::Key(Key::A), KeyCode::Key(Key::B));

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    assert_eq!(report, NONE_REPORT, "TapHold action key, Just tapped");

    let report = update!(state, time(200), (0, 0, false));
    let mut expected = NONE_REPORT;
    expected.keyboard_report = Some(KeyboardReport {
        keycodes: [0x04, 0, 0, 0, 0, 0],
        modifier: 0,
        reserved: 0,
        leds: 0,
    });
    assert_eq!(
        report, expected,
        "TapHold action key, released before tapping term, tap key sent"
    );
}

#[test]
fn taphold_action_other_key_press() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] =
        KeyAction::TapHold(KeyCode::Key(Key::A), KeyCode::Layer(LayerOp::Momentary(1)));
    keymap.layers[1].map[0][1] = KeyAction::Normal(KeyCode::Key(Key::C));

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    assert_eq!(report, NONE_REPORT, "TapHold action key, Just tapped");

    let report = update!(state, time(0), (0, 1, true));
    let mut expected = NONE_REPORT;
    expected.keyboard_report = Some(KeyboardReport {
        keycodes: [0x06, 0, 0, 0, 0, 0],
        modifier: 0,
        reserved: 0,
        leds: 0,
    });
    expected.highest_layer = 1;
    assert_eq!(
        report, expected,
        "In tapping term, but act as hold because another key is pressed"
    );
}

#[test]
fn oneshot_action_mod() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::OneShot(KeyCode::Modifier(Modifier::LCtrl));
    keymap.layers[0].map[0][1] = KeyAction::Normal(KeyCode::Key(Key::A));
    keymap.layers[0].map[0][2] = KeyAction::Normal(KeyCode::Key(Key::B));

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    assert_eq!(report, NONE_REPORT, "One-shot lctrl key is pressed");

    let report = update!(state, time(0), (0, 0, false));
    assert_eq!(report, NONE_REPORT, "One-shot lctrl key is released");

    let report = update!(state, time(0), (0, 1, true));
    let mut expected = NONE_REPORT;
    expected.keyboard_report = Some(KeyboardReport {
        keycodes: [0x04, 0, 0, 0, 0, 0],
        modifier: 0x01,
        reserved: 0,
        leds: 0,
    });
    assert_eq!(report, expected, "Key 'a' is pressed with lctrl modifier");
    let report = update!(state, time(0));
    assert_eq!(
        report, expected,
        "Key 'a' is still pressed with lctrl modifier"
    );

    let report = update!(state, time(0), (0, 1, false));
    assert_eq!(
        report, KEYBOARD_ONLY_REPORT,
        "Oneshot lctrl and 'a' is released"
    );

    let report = update!(state, time(0), (0, 2, true));
    let mut expected = NONE_REPORT;
    expected.keyboard_report = Some(KeyboardReport {
        keycodes: [0x05, 0, 0, 0, 0, 0],
        modifier: 0,
        reserved: 0,
        leds: 0,
    });
    assert_eq!(report, expected, "Key 'b' is pressed without modifier");
}
