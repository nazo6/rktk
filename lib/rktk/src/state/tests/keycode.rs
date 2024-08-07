use super::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn mouse_left_click_key() {
    let mut keymap = EMPTY_KEYMAP;
    keymap[0].map[0][0] = KeyDef::Key(KeyAction::Normal(KeyCode::Mouse(Mouse::Left)));

    let mut state = State::new(keymap, Some(Hand::Left));
    let report = state.update(
        &mut [KeyChangeEventOneHand {
            row: 0,
            col: 0,
            pressed: true,
        }],
        &mut [],
        (0, 0),
        embassy_time::Instant::from_secs(0),
    );

    let mut expected = EMPTY_REPORT;
    expected.mouse_report.as_mut().unwrap().buttons = 0x01;

    assert_eq!(report, expected, "Mouse left button key is pressed");
}

#[test]
fn layer_momentary_key() {
    let mut keymap = EMPTY_KEYMAP;
    keymap[0].map[0][0] = KeyDef::Key(KeyAction::Normal(KeyCode::Layer(LayerOp::Momentary(1))));
    keymap[0].map[0][1] = KeyDef::Key(KeyAction::Normal(KeyCode::Layer(LayerOp::Momentary(2))));

    let mut state = State::new(keymap, Some(Hand::Left));
    let _report = state.update(
        &mut [KeyChangeEventOneHand {
            row: 0,
            col: 0,
            pressed: true,
        }],
        &mut [],
        (0, 0),
        Instant::from_secs(0),
    );
    // layer is updated in the next update
    let report = state.update(&mut [], &mut [], (0, 0), Instant::from_secs(0));

    let mut expected = NONE_REPORT;
    expected.highest_layer = 1;
    assert_eq!(report, expected, "Momentary layer move 1 is pressed");

    let _report = state.update(&mut [], &mut [], (0, 0), Instant::from_secs(0));
    let report = state.update(&mut [], &mut [], (0, 0), Instant::from_secs(0));
    let mut expected = NONE_REPORT;
    expected.highest_layer = 1;
    assert_eq!(report, expected, "Momentary layer move 1 is still pressing");

    let _report = state.update(
        &mut [KeyChangeEventOneHand {
            row: 0,
            col: 0,
            pressed: false,
        }],
        &mut [],
        (0, 0),
        Instant::from_secs(0),
    );
    let report = state.update(&mut [], &mut [], (0, 0), Instant::from_secs(0));
    let mut expected = NONE_REPORT;
    expected.highest_layer = 0;
    assert_eq!(report, expected, "Momentary layer move 1 is released");
}

#[test]
fn layer_toggle_key() {
    let mut keymap = EMPTY_KEYMAP;
    keymap[0].map[0][0] = KeyDef::Key(KeyAction::Normal(KeyCode::Layer(LayerOp::Toggle(1))));

    let mut state = State::new(keymap, Some(Hand::Left));
    let _report = state.update(
        &mut [KeyChangeEventOneHand {
            row: 0,
            col: 0,
            pressed: true,
        }],
        &mut [],
        (0, 0),
        Instant::from_secs(0),
    );
    let report = state.update(&mut [], &mut [], (0, 0), Instant::from_secs(0));
    let mut expected = NONE_REPORT;
    expected.highest_layer = 1;
    assert_eq!(report, expected, "Toggle 1 is pressed");

    let _report = state.update(
        &mut [KeyChangeEventOneHand {
            row: 0,
            col: 0,
            pressed: false,
        }],
        &mut [],
        (0, 0),
        embassy_time::Instant::from_secs(0),
    );
    let report = state.update(&mut [], &mut [], (0, 0), Instant::from_secs(0));
    let mut expected = NONE_REPORT;
    expected.highest_layer = 1;
    assert_eq!(report, expected, "Toggle 1 released, still layer is 1");

    let _report = state.update(
        &mut [KeyChangeEventOneHand {
            row: 0,
            col: 0,
            pressed: true,
        }],
        &mut [],
        (0, 0),
        Instant::from_secs(0),
    );
    let report = state.update(&mut [], &mut [], (0, 0), Instant::from_secs(0));
    let mut expected = NONE_REPORT;
    expected.highest_layer = 0;
    assert_eq!(
        report, expected,
        "Toggle 1 is pressed again, layer is now 0"
    );
}
