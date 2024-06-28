use super::{modifier::Modifier, KeyAction, KeyCode, KeyDef};

/// Press key with shift
#[allow(non_snake_case)]
pub const fn SF(k: KeyDef) -> KeyDef {
    if let KeyDef::Key(KeyAction::Tap(KeyCode::Key(key))) = k {
        KeyDef::Key(KeyAction::Tap(KeyCode::WithModifier(Modifier::LShft, key)))
    } else {
        panic!("Unsupported key type")
    }
}
