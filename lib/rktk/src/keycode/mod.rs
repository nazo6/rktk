//! Keycode/Keymap related things.
//!
//! To archieve flexible key mapping, key definition is bit complex.
//! For example, normal `A` key is defined following:
//! ```rust
//! KeyDef::Key(KeyAction::Tap(KeyCode::Key(Key::A)));
//! ```
//! This is too complex for normal usage, so these normal keys as provided as constants.

use crate::config::static_config::CONFIG;

pub mod key;
pub mod layer;
pub mod macros;
pub mod media;
pub mod modifier;
pub mod mouse;
pub mod special;
pub mod utils;

/// Top-level key definition.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum KeyDef {
    None,
    Inherit,
    Key(KeyAction),
}

/// Defined how key is handled.
///
/// - `Tap`: Normal key press.
/// - `TapHold`: If tapped term is too short, treat as `Tap` (first key is used). If tapped term is longer than `TAP_THRESHOLD`, treat as `Hold` (second key is used).
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum KeyAction {
    Tap(KeyCode),
    TapHold(KeyCode, KeyCode),
}

/// Represents each key.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum KeyCode {
    Key(key::Key),
    Mouse(mouse::Mouse),
    Modifier(modifier::Modifier),
    WithModifier(modifier::Modifier, key::Key),
    Layer(layer::LayerOp),
    Special(special::Special),
    Media(media::Media),
}

/// Inherit key definition from parent layer.
pub const _____: KeyDef = KeyDef::Inherit;
/// No key definition.
pub const XXXXX: KeyDef = KeyDef::None;

pub struct Layer {
    pub map: [[KeyDef; CONFIG.cols * 2]; CONFIG.rows],
    pub arrowball: bool,
}

pub type LayerMap = [[KeyDef; CONFIG.cols * 2]; CONFIG.rows];
