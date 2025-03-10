//! common keymap for test

use crate::keymap::{ComboDefinition, Keymap, Layer, LayerKeymap, TapDanceDefinition};

use super::prelude::*;

#[rustfmt::skip]
/// Auto mouse layer
pub const EMPTY_LAYER: LayerKeymap<ROWS, COLS> = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const EMPTY_KEYMAP: Keymap<LAYER_COUNT, ROWS, COLS, ENC_COUNT, 2, 4, 2, 3> = Keymap {
    layers: [
        Layer {
            keymap: EMPTY_LAYER,
            ..Layer::const_default()
        },
        Layer {
            keymap: EMPTY_LAYER,
            ..Layer::const_default()
        },
        Layer {
            keymap: EMPTY_LAYER,
            ..Layer::const_default()
        },
        Layer {
            keymap: EMPTY_LAYER,
            ..Layer::const_default()
        },
        Layer {
            keymap: EMPTY_LAYER,
            ..Layer::const_default()
        },
    ],
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
    ],
    combo: [
        Some(ComboDefinition {
            src: [Some(KeyCode::Key(Key::G)), Some(KeyCode::Key(Key::H)), None],
            dst: KeyCode::Key(Key::I),
        }),
        None,
    ],
};
