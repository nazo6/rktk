use rktk::config::keymap::{
    prelude::*,
    Keymap, Layer, LayerKeymap,
};

#[rustfmt::skip]
const L0: LayerKeymap = [
    [  G    , H     , _____ ],
    [  D    , E     , F     ],
    [  A    , B     , C     ],
];

pub const KEYMAP: Keymap = Keymap {
    layers: [
        Layer {
            keymap: L0,
            ..Layer::const_default()
        },
        Layer::const_default(),
        Layer::const_default(),
        Layer::const_default(),
        Layer::const_default(),
    ],
    ..Keymap::const_default()
};
