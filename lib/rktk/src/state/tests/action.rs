use super::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn normal_action() {
    let mut keymap = EMPTY_KEYMAP;
    keymap[0].map[0][0] = KeyDef::Key(KeyAction::Normal(KeyCode::Key(Key::A)));

    let now = embassy_time::Instant::from_secs(0);
    let mut state = super::super::State::new(keymap, Some(Hand::Left));
    let report = state.update(
        &mut [KeyChangeEventOneHand {
            col: 0,
            row: 0,
            pressed: true,
        }],
        &mut [],
        (0, 0),
        now,
    );

    let mut expected = EMPTY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes = [0x04, 0, 0, 0, 0, 0];

    assert_eq!(report, expected, "Normal action key 'a' is pressed");
}

#[test]
fn normal2_action() {
    let mut keymap = EMPTY_KEYMAP;
    keymap[0].map[0][0] = KeyDef::Key(KeyAction::Normal2(
        KeyCode::Key(Key::A),
        KeyCode::Key(Key::B),
    ));

    let now = embassy_time::Instant::from_secs(0);
    let mut state = super::super::State::new(keymap, Some(Hand::Left));
    let report = state.update(
        &mut [KeyChangeEventOneHand {
            col: 0,
            row: 0,
            pressed: true,
        }],
        &mut [],
        (0, 0),
        now,
    );

    let mut expected = EMPTY_REPORT;
    expected.keyboard_report.as_mut().unwrap().keycodes = [0x04, 0x05, 0, 0, 0, 0];

    assert_eq!(
        report, expected,
        "Normal2 action key 'a' and 'b' are pressed"
    );
}

#[test]
fn taphold_action_hold() {
    let mut keymap = EMPTY_KEYMAP;
    keymap[0].map[0][0] = KeyDef::Key(KeyAction::TapHold(
        KeyCode::Key(Key::A),
        KeyCode::Key(Key::B),
    ));
    let now = embassy_time::Instant::from_secs(0);

    let mut state = super::super::State::new(keymap, Some(Hand::Left));

    let report = state.update(
        &mut [KeyChangeEventOneHand {
            col: 0,
            row: 0,
            pressed: true,
        }],
        &mut [],
        (0, 0),
        now,
    );

    assert_eq!(report, EMPTY_REPORT, "TapHold action key, Just tapped");

    let now = embassy_time::Instant::from_millis(50);
    let report = state.update(&mut [], &mut [], (0, 0), now);
    assert_eq!(
        report, NONE_REPORT,
        "TapHold action key, Sill in tapping term"
    );

    let now = embassy_time::Instant::from_millis(1000);
    let report = state.update(&mut [], &mut [], (0, 0), now);

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
    keymap[0].map[0][0] = KeyDef::Key(KeyAction::TapHold(
        KeyCode::Key(Key::A),
        KeyCode::Key(Key::B),
    ));
    let now = embassy_time::Instant::from_secs(0);

    let mut state = super::super::State::new(keymap, Some(Hand::Left));

    let report = state.update(
        &mut [KeyChangeEventOneHand {
            col: 0,
            row: 0,
            pressed: true,
        }],
        &mut [],
        (0, 0),
        now,
    );
    assert_eq!(report, EMPTY_REPORT, "TapHold action key, Just tapped");

    let now = embassy_time::Instant::from_millis(200);
    let report = state.update(
        &mut [KeyChangeEventOneHand {
            col: 0,
            row: 0,
            pressed: false,
        }],
        &mut [],
        (0, 0),
        now,
    );
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
