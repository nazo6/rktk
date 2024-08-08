use super::{modifier::Modifier, KeyAction, KeyCode, KeyDef};

/// Press key with shift
#[allow(non_snake_case)]
pub const fn SF(k: KeyDef) -> KeyDef {
    if let KeyDef::Key(KeyAction::Normal(KeyCode::Key(key))) = k {
        KeyDef::Key(KeyAction::Normal2(
            KeyCode::Modifier(Modifier::LShft),
            KeyCode::Key(key),
        ))
    } else {
        panic!("Unsupported key type")
    }
}
