//! Key action and keycode definitions.
//!
//! A key is represented by a three-layer structure: `KeyAction` → `KeyCode` → `Key`.
//! For example, a key action that sends A on Tap and Shift on Hold could be defined as follows
//! ```
//! # use rktk_keymanager::keycode::prelude::*;
//! const ACTION: KeyAction = KeyAction::TapHold(KeyCode::Key(Key::A),
//! KeyCode::Modifier(Modifier::LShft));
//! ```
//!
//! This hierarchical structure allows any key to be used for any action.
//! For example, QMK only allows Mod-Tap and Layer-Tap as TapHolds.
//! However, it requires more bytes to represent one key.
//!
//! For convenience, keycodes are defined as constants with normal keyaction like this:
//! ```
//! # use rktk_keymanager::keycode::prelude::*;
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
#[derive(Copy)]
pub enum KeyAction {
    /// Inherit key definition from parent layer.
    Inherit,
    /// Normal key
    Normal(KeyCode),
    /// Normal key with secondary key. These keys are sent together.
    Normal2(KeyCode, KeyCode),
    /// Tap-Hold key (tap, hold)
    ///
    /// If key is pressed and released in [`KeyResolverConfig::tap_hold`](crate::interface::state::config::KeyResolverConfig::tap_hold), tap key is sent.
    /// If key is pressed and held longer, hold key is sent. Also, `rktk-keymanager` emulates qmk's `HOLD_ON_OTHER_KEY_PRESS` feature.
    /// If another key is pressed while holding this key, even before `tap_threshold`, hold key is sent.
    TapHold(KeyCode, KeyCode),
    /// One-shot key
    ///
    /// After pressing this key, the key will be sent together with the next key press.
    OneShot(KeyCode),
    /// Tap-dance (id)
    ///
    /// Execute tap-dance with specified id. TapDance can be configured in [`KeyResolverConfig`](crate::interface::state::config::KeyResolverConfig).
    TapDance(u8),
}

impl KeyAction {
    pub const fn const_default() -> Self {
        Self::Inherit
    }
}

impl Default for KeyAction {
    fn default() -> Self {
        Self::const_default()
    }
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
    Custom(u8),
}

/// Inherit key: `KeyAction::Inherit`
pub const _____: KeyAction = KeyAction::Inherit;
pub const ____: KeyAction = KeyAction::Inherit;
pub const ___: KeyAction = KeyAction::Inherit;
pub const __: KeyAction = KeyAction::Inherit;

/// None key: `KeyAction::Normal(KeyCode::None)`
pub const XXXXX: KeyAction = KeyAction::Normal(KeyCode::None);
