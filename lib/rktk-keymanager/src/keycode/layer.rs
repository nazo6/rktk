#![allow(non_snake_case)]

use macro_rules_attribute::apply;

use super::{KeyAction, KeyCode};
use crate::macros::common_derive;

/// Keycode for layer operations.
/// - `Move`: Move to the layer.
/// - `Toggle`: Move layer only while key is pressed.
#[apply(common_derive)]
#[derive(Copy)]
pub enum LayerOp {
    Momentary(u8),
    Toggle(u8),
}

pub const fn MO(n: u8) -> KeyAction {
    // assert!(n < LAYER_NUM);
    KeyAction::Normal(KeyCode::Layer(LayerOp::Momentary(n)))
}

pub const fn TG(n: u8) -> KeyAction {
    // assert!(n < LAYER_NUM);
    KeyAction::Normal(KeyCode::Layer(LayerOp::Toggle(n)))
}
