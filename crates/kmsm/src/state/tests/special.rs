use super::prelude::*;
use pretty_assertions::assert_eq;

#[test]
pub fn lock() {
    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].keymap[0][0] = KeyAction::Normal(KeyCode::Special(Special::LockTg));
    keymap.layers[0].keymap[0][1] = KeyAction::Normal(KeyCode::Key(Key::A));

    let mut state = new_state(keymap);

    dbg!("T1");
    let report = update!(state, time(0), (0, 1, true));
    assert_eq!(
        report,
        report_with_keycodes([0x04, 0, 0, 0, 0, 0]),
        "Not locked. Key 'a' pressed"
    );

    dbg!("T2");
    let report = update!(state, time(10), (0, 0, true));
    assert_eq!(report, NONE_REPORT, "Lock pressed. Still Key 'a' pressed");
    dbg!("T3");
    let report = update!(state, time(20), (0, 0, false));
    assert_eq!(report, NONE_REPORT, "Lock released. Still Key 'a' pressed");

    dbg!("T4");
    let report = update!(state, time(30), (0, 1, false));
    assert_eq!(report, KEYBOARD_ONLY_REPORT, "Locked. Key 'a' released");

    dbg!("T5");
    let report = update!(state, time(40), (0, 1, true));
    assert_eq!(report, NONE_REPORT, "Locked. Key 'a' pressed, but not sent");
    dbg!("T6");
    let report = update!(state, time(50), (0, 1, false));
    assert_eq!(report, KEYBOARD_ONLY_REPORT, "Locked. Key 'a' released.");

    dbg!("T7");
    let report = update!(state, time(60), (0, 0, true));
    assert_eq!(report, NONE_REPORT, "Lock pressed");
    dbg!("T8");
    let report = update!(state, time(70), (0, 0, false));
    assert_eq!(report, NONE_REPORT, "Lock released");

    let report = update!(state, time(80), (0, 1, true));
    assert_eq!(
        report,
        report_with_keycodes([0x04, 0, 0, 0, 0, 0]),
        "Unlocked. Key 'a' pressed"
    );
    let report = update!(state, time(90), (0, 1, false));
    assert_eq!(report, KEYBOARD_ONLY_REPORT, "Unlocked. Key 'a' released");
}
