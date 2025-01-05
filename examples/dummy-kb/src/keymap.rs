use rktk::config::keymap::{key_manager::keycode::_____, Keymap, Layer, LayerKeymap};

#[rustfmt::skip]
const L0: LayerKeymap = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const KEYMAP: Keymap = Keymap {
    encoder_keys: [],
    layers: [
        Layer {
            keymap: L0,
            arrowmouse: false,
        },
        Layer {
            keymap: L0,
            arrowmouse: false,
        },
        Layer {
            keymap: L0,
            arrowmouse: false,
        },
        Layer {
            keymap: L0,
            arrowmouse: true,
        },
        Layer {
            keymap: L0,
            arrowmouse: true,
        },
    ],
    tap_dance: [None, None],
    combo: [None, None],
};
