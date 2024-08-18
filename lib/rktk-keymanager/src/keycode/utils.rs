use super::{modifier::Modifier, KeyAction, KeyCode};

/// Press key with shift
#[allow(non_snake_case)]
pub const fn SF(k: KeyAction) -> KeyAction {
    if let KeyAction::Normal(KeyCode::Key(key)) = k {
        KeyAction::Normal2(KeyCode::Modifier(Modifier::LShft), KeyCode::Key(key))
    } else {
        panic!("Unsupported key type")
    }
}
