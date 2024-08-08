//! common keymap for test

use crate::{Layer, LayerMap};

use super::prelude::*;

#[rustfmt::skip]
/// Auto mouse layer
const EMPTY_LAYER: LayerMap<ROWS, COLS> = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const EMPTY_KEYMAP: [Layer<ROWS, COLS>; LAYER_COUNT] = [
    Layer {
        map: EMPTY_LAYER,
        arrowball: false,
    },
    Layer {
        map: EMPTY_LAYER,
        arrowball: false,
    },
    Layer {
        map: EMPTY_LAYER,
        arrowball: false,
    },
    Layer {
        map: EMPTY_LAYER,
        arrowball: false,
    },
    Layer {
        map: EMPTY_LAYER,
        arrowball: true,
    },
];
