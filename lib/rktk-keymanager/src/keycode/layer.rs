//! Layer operation keys

#![allow(non_snake_case)]

use macro_rules_attribute::apply;

use super::{KeyAction, KeyCode};
use crate::macros::common_derive;

/// Keycode for layer operations.
#[apply(common_derive)]
#[derive(Copy)]
pub enum LayerOp {
    /// Momentary activates the specified layer.
    Momentary(u8),
    /// Toggles the state of the specified layer.
    Toggle(u8),
}

pub const fn MO(n: u8) -> KeyAction {
    KeyAction::Normal(KeyCode::Layer(LayerOp::Momentary(n)))
}

pub const fn TG(n: u8) -> KeyAction {
    KeyAction::Normal(KeyCode::Layer(LayerOp::Toggle(n)))
}
