use crate::state::tests::prelude::*;

#[test]
fn tap_dance_tap1() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::TapDance(0);

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    let expected = NONE_REPORT;
    assert_eq!(
        report, expected,
        "TapDance key pressed. nothing happens yet"
    );
    let report = update!(state, time(0), (0, 0, false));
    let expected = NONE_REPORT;
    assert_eq!(
        report, expected,
        "TapDance key released. nothing happens yet"
    );

    let report = update!(state, time(50), (0, 0, false));
    let expected = NONE_REPORT;
    assert_eq!(
        report, expected,
        "TapDance key released but dance tapping term not exceeded. nothing happens yet."
    );

    let report = state.update(&mut [], (0, 0), &[], time(150));
    let mut expected = KEYBOARD_ONLY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes = [0x04, 0, 0, 0, 0, 0];
    assert_eq!(
        report, expected,
        "tap dance term exceeded. tap key should be sent"
    );
}

#[test]
fn tap_dance_hold1() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::TapDance(0);

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    let expected = NONE_REPORT;
    assert_eq!(
        report, expected,
        "TapDance key pressed. nothing happens yet"
    );

    let report = state.update(&mut [], (0, 0), &[], time(600));
    let mut expected = KEYBOARD_ONLY_REPORT;
    expected.keyboard_report.as_mut().unwrap().modifier = 0x01;
    assert_eq!(
        report, expected,
        "tapping term exceeded. hold key should be sent"
    );

    let report = update!(state, time(0), (0, 0, false));
    let expected = KEYBOARD_ONLY_REPORT;
    assert_eq!(report, expected, "key released");
}

#[test]
fn tap_dance_tap2() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::TapDance(0);

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    let expected = NONE_REPORT;
    assert_eq!(
        report, expected,
        "TapDance key pressed for the first time. nothing happens yet"
    );
    let report = update!(state, time(0), (0, 0, false));
    let expected = NONE_REPORT;
    assert_eq!(
        report, expected,
        "TapDance key released for the first time. nothing happens yet"
    );

    let report = update!(state, time(50), (0, 0, true));
    let expected = NONE_REPORT;
    assert_eq!(
        report, expected,
        "TapDance key pressed one more. nothing happens yet"
    );

    let report = update!(state, time(50), (0, 0, false));
    let expected = NONE_REPORT;
    assert_eq!(
        report, expected,
        "TapDance key released one more. nothing happens yet"
    );

    let report = state.update(&mut [], (0, 0), &[], time(250));
    let mut expected = KEYBOARD_ONLY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes = [0x05, 0, 0, 0, 0, 0];
    assert_eq!(
        report, expected,
        "tap dance term exceeded. tap key should be sent"
    );
}

#[test]
fn tap_dance_hold2() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::TapDance(0);

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let _ = update!(state, time(0), (0, 0, true));
    let _ = update!(state, time(0), (0, 0, false));

    let _ = update!(state, time(50), (0, 0, true));

    let report = state.update(&mut [], (0, 0), &[], time(600));
    assert_eq!(report.highest_layer, 1, "hold2");
}

#[test]
fn tap_dance_tap3() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::TapDance(0);

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let _ = update!(state, time(0), (0, 0, true));
    let _ = update!(state, time(0), (0, 0, false));

    let _ = update!(state, time(50), (0, 0, true));
    let _ = update!(state, time(50), (0, 0, false));

    let _ = update!(state, time(100), (0, 0, true));
    let _ = update!(state, time(100), (0, 0, false));

    let report = state.update(&mut [], (0, 0), &[], time(700));
    assert_eq!(report.highest_layer, 2, "tap3");
}
