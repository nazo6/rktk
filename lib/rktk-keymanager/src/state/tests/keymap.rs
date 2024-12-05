//! common keymap for test

use crate::keymap::{Keymap, Layer, LayerMap, TapDanceDefinition};

use super::prelude::*;

#[rustfmt::skip]
/// Auto mouse layer
pub const EMPTY_LAYER: LayerMap<ROWS, COLS> = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const EMPTY_KEYMAP: Keymap<LAYER_COUNT, ROWS, COLS, ENC_COUNT> = Keymap {
    layers: [
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: false,
        },
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: false,
        },
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: false,
        },
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: false,
        },
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: true,
        },
    ],
    encoder_keys: [(KeyCode::None, KeyCode::None)],
    tap_dance: [
        Some(TapDanceDefinition {
            tap: [
                Some(KeyCode::Key(Key::A)),
                Some(KeyCode::Key(Key::B)),
                Some(KeyCode::Layer(LayerOp::Toggle(2))),
                None,
            ],
            hold: [
                Some(KeyCode::Modifier(Modifier::LCtrl)),
                Some(KeyCode::Layer(LayerOp::Momentary(1))),
                None,
                None,
            ],
        }),
        None,
        None,
        None,
    ],
};
