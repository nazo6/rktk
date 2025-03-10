use super::prelude::*;
use pretty_assertions::assert_eq;

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
