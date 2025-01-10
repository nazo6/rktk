use rktk::config::keymap::{
    key_manager::{
        keycode::{key::*, layer::*, media::*, modifier::*, mouse::*, special::*, utils::*, *},
        keymap::TapDanceDefinition,
    },
    Keymap, Layer, LayerKeymap,
};

const L2SPC: KeyAction = KeyAction::TapHold(
    KeyCode::Key(Key::Enter),
    KeyCode::Layer(LayerOp::Momentary(2)),
);

#[rustfmt::skip]
const L0: LayerKeymap = [
    [  TAB  , Q     , W     , E     , R     , T     , /**/  Y     , U     , I     , O     , P    , MINUS],
    [  ESC  , A     , S     , D     , F     , G     , /**/  H     , J     , K     , L     , SCLN , QUOTE],
    [ L_SHFT, Z     , X     , C     , V     , B     , /**/  N     , M     , COMM  , DOT   , SLASH, BSLSH],
    [ L_CTRL, L_GUI , TG(2) , L_ALT , L2SPC , SPACE , /**/  BS    , ENTER , _____ , _____ ,R_SHFT,R_CTRL],
];

#[rustfmt::skip]
/// Auto mouse layer
const L1: LayerKeymap = [
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ ,M_LEFT ,MO_SCRL,M_RIGHT, _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ ,M_BACK ,M_MIDDLE,M_FORWARD,_____,_____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ , _____ , _____ , _____ , _____ , _____ ],
];

#[rustfmt::skip]
/// Mouse layer
const L2: LayerKeymap = [
    [ _____ , _____ , INSERT, HOME  , PGUP  , _____ ,  /**/  LEFT  , DOWN  , UP    , RIGHT , _____ , F12   ],
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ ,M_LEFT ,MO_SCRL,M_RIGHT, _____ , VOLUP ],
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ ,M_BACK ,M_MIDDLE,M_FORWARD,_____,VOLDN ],
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  DELETE, _____ , _____ , _____ , PRTSC , _____ ],
];

#[rustfmt::skip]
const L3: LayerKeymap = [
    [ _____ , _____ , KP7   , KP8   , KP9   , _____ ,  /**/  SF(D1), SF(D2), SF(D3), SF(D4), SF(D5), _____ ],
    [ _____ , _____ , KP4   , KP5   , KP6   , _____ ,  /**/  SF(D6), SF(D7), SF(D8), SF(D9), SF(D0), _____ ],
    [ _____ , _____ , KP1   , KP2   , KP3   , _____ ,  /**/  QUOTE,SF(QUOTE),EQUAL,SF(EQUAL), _____, _____ ],
    [ _____ , _____ , KP0   , _____ , _____ , _____ ,  /**/  _____ , _____ , _____ , _____ , _____ , _____ ],
];

#[rustfmt::skip]
const L4: LayerKeymap = [
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ ,  /**/  _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const KEYMAP: Keymap = Keymap {
    encoder_keys: [],
    layers: [
        Layer {
            keymap: L0,
            arrowmouse: false,
        },
        Layer {
            keymap: L1,
            arrowmouse: false,
        },
        Layer {
            keymap: L2,
            arrowmouse: false,
        },
        Layer {
            keymap: L3,
            arrowmouse: true,
        },
        Layer {
            keymap: L4,
            arrowmouse: true,
        },
    ],
    tap_dance: [
        Some(TapDanceDefinition {
            tap: [
                Some(KeyCode::Key(Key::RightBracket)),
                Some(KeyCode::Layer(LayerOp::Toggle(2))),
                None,
                None,
            ],
            hold: [None, None, None, None],
        }),
        None,
    ],
    combo: [None, None],
};
