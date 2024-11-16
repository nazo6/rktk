//! common keymap for test

use crate::{Keymap, Layer, LayerMap};

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
};
