use super::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn layer_change_during_press() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].keymap[0][0] = KeyAction::Normal(KeyCode::Key(Key::A));
    keymap.layers[1].keymap[0][0] = KeyAction::Normal(KeyCode::Mouse(Mouse::MLeft));

    let mut state = new_state(keymap);
    let _ = update!(state, time(0));

    let report = update!(state, time(0), (0, 0, true));
    assert_eq!(
        report,
        report_with_keycodes([0x04, 0, 0, 0, 0, 0]),
        "Key 'A' at layer 0 is pressed"
    );

    let _report = state.update(InputEvent::Mouse((20, 10)), time(50));
    assert_eq!(
        state.inner().shared.layer_active,
        [false, true, false, false, false],
        "Aml activated"
    );

    let report = update!(state, time(200), (0, 0, false));
    assert_eq!(
        report.keyboard_report.unwrap().keycodes,
        [0, 0, 0, 0, 0, 0],
        "Key released"
    );

    let _report = state.update(InputEvent::None, time(1000));
    assert_eq!(
        state.inner().shared.layer_active,
        [false, false, false, false, false],
        "Aml deactivated"
    );
}
