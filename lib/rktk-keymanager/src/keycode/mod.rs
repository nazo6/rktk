//! Keycode/Keymap related things.
//!
//! To archieve flexible key mapping, key definition is bit complex.
//! For example, normal `A` key is defined following:
//! ```ignore
//! KeyDef::Key(KeyAction::Tap(KeyCode::Key(Key::A)));
//! ```
//! This is too complex for normal usage, so these normal keys as provided as constants.

pub mod key;
pub mod layer;
pub mod macros;
pub mod media;
pub mod modifier;
pub mod mouse;
pub mod special;
pub mod utils;

/// Top-level key definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum KeyDef {
    None,
    Inherit,
    Key(KeyAction),
}

/// Defined how key is handled.
///
/// - `Normal`: Normal key press.
/// - `Normal2`: Press key with another key.
/// - `TapHold`: If tapped term is too short, treat as `Tap` (first key is used). If tapped term is longer than `TAP_THRESHOLD`, treat as `Hold` (second key is used).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum KeyAction {
    Normal(KeyCode),
    Normal2(KeyCode, KeyCode),
    TapHold(KeyCode, KeyCode),
    OneShot(KeyCode),
    // In future add more actions like:
    // TapDance(TapDanceId),
}

/// Represents each key.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum KeyCode {
    Key(key::Key),
    Mouse(mouse::Mouse),
    Modifier(modifier::Modifier),
    Layer(layer::LayerOp),
    Special(special::Special),
    Media(media::Media),
}

/// Inherit key definition from parent layer.
pub const _____: KeyDef = KeyDef::Inherit;
/// No key definition.
pub const XXXXX: KeyDef = KeyDef::None;
