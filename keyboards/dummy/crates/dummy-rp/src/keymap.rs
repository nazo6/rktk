use rktk::config::keymap::{Keymap, Layer, LayerKeymap, keymanager::keycode::_____};

#[rustfmt::skip]
const L0: LayerKeymap = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const KEYMAP: Keymap = Keymap {
    layers: [
        Layer {
            keymap: L0,
            ..Layer::const_default()
        },
        Layer {
            keymap: L0,
            ..Layer::const_default()
        },
        Layer {
            keymap: L0,
            ..Layer::const_default()
        },
        Layer {
            keymap: L0,
            ..Layer::const_default()
        },
        Layer {
            keymap: L0,
            ..Layer::const_default()
        },
    ],
    ..Keymap::const_default()
};
