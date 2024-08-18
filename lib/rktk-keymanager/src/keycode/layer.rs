#![allow(non_snake_case)]

use super::{KeyAction, KeyCode};

/// Keycode for layer operations.
/// - `Move`: Move to the layer.
/// - `Toggle`: Move layer only while key is pressed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
