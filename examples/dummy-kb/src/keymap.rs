use rktk::config::keymap::{keymanager::keycode::_____, Keymap, Layer, LayerKeymap};

#[rustfmt::skip]
const L0: LayerKeymap = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const KEYMAP: Keymap = Keymap {
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
    ..Keymap::const_default()
};
