//! common keymap for test

use crate::config::static_config::CONFIG;
use crate::keycode::*;

#[rustfmt::skip]
/// Auto mouse layer
const EMPTY_LAYER: LayerMap = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const EMPTY_KEYMAP: [Layer; CONFIG.layer_count] = [
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
