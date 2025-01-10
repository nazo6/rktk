//! Key action and keycode definitions.
//!
//! A key is represented by a three-layer structure: `KeyAction` → `KeyCode` → `Key`.
//! For example, the definition of a normal key `A` is as follows.
//! ```ignore
//! KeyAction::Normal(KeyCode::Key(Key::A))
//! ```
//! This hierarchical structure allows any key to be used for any action.
//! For example, QMK only allows Mod-Tap and Layer-Tap as TapHolds.
//! However, it requires more bytes to represent one key.
//!
//! For convenience, keycodes are defined as constants with normal keyaction like this:
//! ```ignore
//! const A: KeyAction = KeyAction::Normal(KeyCode::Key(Key::A));
//! ```
//! You can use these constants to define keymap.

use macro_rules_attribute::apply;

use crate::macros::common_derive;

pub mod key;
pub mod layer;
pub mod media;
pub mod modifier;
pub mod mouse;
pub mod special;
pub mod utils;

/// Convenient prelude of keycodes for defining keymaps
pub mod prelude {
    pub use super::{key::*, layer::*, media::*, modifier::*, mouse::*, special::*, utils::*, *};
}

/// Represents key action.
#[apply(common_derive)]
#[derive(Copy, Default)]
pub enum KeyAction {
    /// Inherit key definition from parent layer.
    #[default]
    Inherit,
    /// Normal key
    Normal(KeyCode),
    /// Normal key with secondary key. These keys are sent together.
    Normal2(KeyCode, KeyCode),
    /// Tap-Hold key (tap, hold)
    ///
    /// If key is pressed and released in [`tap_threshold`](crate::state::config::KeyResolverConfig::tap_threshold), tap key is sent.
    /// If key is pressed and held longer, hold key is sent. Also, `rktk-keymanager` emulates qmk's `HOLD_ON_OTHER_KEY_PRESS` feature.
    /// If another key is pressed while holding this key, even before `tap_threshold`, hold key is sent.
    TapHold(KeyCode, KeyCode),
    /// One-shot key
    ///
    /// After pressing this key, the key will be sent together with the next key press.
    OneShot(KeyCode),
    /// Tap-dance (id)
    ///
    /// Execute tap-dance with specified id. TapDance can be configured in [`KeyResolverConfig`](crate::state::config::KeyResolverConfig).
    TapDance(u8),
}

/// Represents each key.
#[apply(common_derive)]
#[derive(Copy)]
pub enum KeyCode {
    None,
    /// Normal key
    Key(key::Key),
    /// Mouse key (button)
    Mouse(mouse::Mouse),
    /// Modifier key
    Modifier(modifier::Modifier),
    /// Layer operation key
    Layer(layer::LayerOp),
    /// Special key
    Special(special::Special),
    /// Media key
    Media(media::Media),
}

/// Inherit key: `KeyAction::Inherit`
pub const _____: KeyAction = KeyAction::Inherit;
pub const ____: KeyAction = KeyAction::Inherit;
pub const ___: KeyAction = KeyAction::Inherit;
pub const __: KeyAction = KeyAction::Inherit;

/// None key: `KeyAction::Normal(KeyCode::None)`
pub const XXXXX: KeyAction = KeyAction::Normal(KeyCode::None);
