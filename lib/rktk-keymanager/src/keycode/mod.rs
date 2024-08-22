//! Keycode/Keymap related things.
//!
//! To archieve flexible key mapping, key definition is bit complex.
//! For example, normal `A` key is defined following:
//! ```ignore
//! KeyDef::Key(KeyAction::Tap(KeyCode::Key(Key::A)));
//! ```
//! This is too complex for normal usage, so these normal keys as provided as constants.

use macro_rules_attribute::apply;

use crate::macros::common_derive;

pub mod key;
pub mod layer;
pub mod media;
pub mod modifier;
pub mod mouse;
pub mod special;
pub mod utils;

/// Defined how key is handled.
///
/// - `Normal`: Normal key press.
/// - `Normal2`: Press key with another key.
/// - `TapHold`: If tapped term is too short, treat as `Tap` (first key is used). If tapped term is longer than `TAP_THRESHOLD`, treat as `Hold` (second key is used).
#[apply(common_derive)]
#[derive(Copy)]
pub enum KeyAction {
    Inherit,
    Normal(KeyCode),
    Normal2(KeyCode, KeyCode),
    TapHold(KeyCode, KeyCode),
    OneShot(KeyCode),
    TapDance(u8),
}

/// Represents each key.
#[apply(common_derive)]
#[derive(Copy)]
pub enum KeyCode {
    None,
    Key(key::Key),
    Mouse(mouse::Mouse),
    Modifier(modifier::Modifier),
    Layer(layer::LayerOp),
    Special(special::Special),
    Media(media::Media),
}

/// Inherit key definition from parent layer.
pub const _____: KeyAction = KeyAction::Inherit;
/// No key definition.
pub const XXXXX: KeyAction = KeyAction::Normal(KeyCode::None);
