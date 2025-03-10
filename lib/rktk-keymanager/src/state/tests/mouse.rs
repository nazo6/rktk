use super::prelude::*;
use pretty_assertions::assert_eq;

pub const MOUSE_ONLY_REPORT: Report = Report {
    keyboard_report: None,
    mouse_report: Some(MouseReport {
        buttons: 0,
        x: 0,
        y: 0,
        wheel: 0,
        pan: 0,
    }),
    media_keyboard_report: None,
    highest_layer: 1,
};

#[test]
pub fn aml() {
    let mut state = new_state(EMPTY_KEYMAP);

    let report = state.update(InputEvent::Mouse((1, 1)), time(0));

    assert_eq!(
        report.highest_layer, 0,
        "Mouse moved a bit, aml not activated"
    );

    let report = state.update(InputEvent::Mouse((1, 1)), time(0));
    assert_eq!(
        report.highest_layer, 0,
        "Mouse moved a bit again, aml not activated"
    );

    let _ = state.update(InputEvent::Mouse((1, 1)), time(0));
    let report = update!(state, time(0));
    assert_eq!(
        report.highest_layer, 1,
        "Mouse moved once again, aml activated"
    );
}

#[test]
pub fn aml_reset() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].keymap[0][0] = KeyAction::Normal(KeyCode::Special(Special::AmlReset));

    let mut state = new_state(keymap);

    let _ = state.update(InputEvent::Mouse((5, 5)), time(0));
    let report = update!(state, time(0));
    assert_eq!(report.highest_layer, 1, "Mouse moved, aml activated");

    let _ = update!(state, time(0), (0, 0, true));
    let report = update!(state, time(0));
    assert_eq!(
        report.highest_layer, 0,
        "Aml reset key pressed. aml deactivated"
    );
}

#[test]
pub fn scroll_remained() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].keymap[0][0] = KeyAction::Normal(KeyCode::Special(Special::MoScrl));

    let mut state = new_state(keymap);

    let report = update!(state, time(0), (0, 0, true));
    assert_eq!(report, NONE_REPORT, "MoScrl pressed");

    let report = state.update(InputEvent::Mouse((20, 10)), time(50));
    let mut expected = MOUSE_ONLY_REPORT;
    expected.mouse_report.as_mut().unwrap().pan = 1;
    expected.mouse_report.as_mut().unwrap().wheel = 0;
    assert_eq!(report, expected, "x:20, y:10 -> pan:1, wheel:0");

    let report = state.update(InputEvent::Mouse((0, 5)), time(80));
    let mut expected = MOUSE_ONLY_REPORT;
    expected.mouse_report.as_mut().unwrap().wheel = -1;
    assert_eq!(report, expected, "Consume remaining pan");
}
