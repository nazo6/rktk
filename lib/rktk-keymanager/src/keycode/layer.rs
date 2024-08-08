#![allow(non_snake_case)]

use super::super::keycode::KeyDef;

use super::{KeyAction, KeyCode};

/// Keycode for layer operations.
/// - `Move`: Move to the layer.
/// - `Toggle`: Move layer only while key is pressed.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LayerOp {
    Momentary(u8),
    Toggle(u8),
}

pub const fn MO(n: u8) -> KeyDef {
    // assert!(n < LAYER_NUM);
    KeyDef::Key(KeyAction::Normal(KeyCode::Layer(LayerOp::Momentary(n))))
}

pub const fn TG(n: u8) -> KeyDef {
    // assert!(n < LAYER_NUM);
    KeyDef::Key(KeyAction::Normal(KeyCode::Layer(LayerOp::Toggle(n))))
}
