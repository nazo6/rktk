use rktk::config::keymap::{Keymap, Layer, LayerKeymap, prelude::*};

#[rustfmt::skip]
const L0: LayerKeymap = [
    [  G    , H     , _____ , MUTE  ],
    [  D    , E     , F     , PLAY  ],
    [  A    , B     , C     , NEXT_TRACK ],
    [ _____ , _____ , _____ , PREV_TRACK ],
];

pub const KEYMAP: Keymap = Keymap {
    layers: [
        Layer {
            keymap: L0,
            encoder_keys: [
                (
                    Some(KeyCode::Media(Media::VolumeDecrement)),
                    Some(KeyCode::Media(Media::VolumeIncrement)),
                ),
                (
                    Some(KeyCode::Media(Media::VolumeDecrement)),
                    Some(KeyCode::Media(Media::VolumeIncrement)),
                ),
            ],
            arrow_mouse: false,
        },
        Layer::const_default(),
        Layer::const_default(),
        Layer::const_default(),
        Layer::const_default(),
    ],
    ..Keymap::const_default()
};
