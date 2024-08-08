use rktk::config::static_config::CONFIG;
use rktk::keycode::*;
use rktk::keycode::{key::*, layer::*, media::*, modifier::*, mouse::*, special::*, utils::*};

const L2ENTER: KeyDef = KeyDef::Key(KeyAction::TapHold(
    KeyCode::Key(Key::Enter),
    KeyCode::Layer(LayerOp::Move(2)),
));

const L2SPC: KeyDef = KeyDef::Key(KeyAction::TapHold(
    KeyCode::Key(Key::Enter),
    KeyCode::Layer(LayerOp::Move(2)),
));

const L3SPC: KeyDef = KeyDef::Key(KeyAction::TapHold(
    KeyCode::Key(Key::Enter),
    KeyCode::Layer(LayerOp::Move(3)),
));

const L4GRV: KeyDef = KeyDef::Key(KeyAction::TapHold(
    KeyCode::Key(Key::Grave),
    KeyCode::Layer(LayerOp::Move(4)),
));

#[rustfmt::skip]
const L0: LayerMap = [
    [ L4GRV , D1    , D2    , D3    , D4    , D5    , _____ , /**/ _____ , D6    , D7    , D8    , D9    , D0   , EQUAL ],
    [  TAB  , Q     , W     , E     , R     , T     , _____ , /**/ _____ , Y     , U     , I     , O     , P    , MINUS],
    [  ESC  , A     , S     , D     , F     , G     , _____ , /**/ _____ , H     , J     , K     , L     , SCLN , QUOTE],
    [ L_SHFT, Z     , X     , C     , V     , B     , LBRC  , /**/ RBRC  , N     , M     , COMM  , DOT   , SLASH, BSLSH],
    [ L_CTRL, L_GUI , TG(2) , L_ALT , L3SPC , L2SPC , SPACE , /**/ BS    ,L2ENTER, _____ , _____ , _____ ,R_SHFT,R_CTRL],
];

#[rustfmt::skip]
/// Auto mouse layer
const L1: LayerMap = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , M_L   ,MO_SCRL, M_R   , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , M_BCK , M_MID , M_FWD , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

#[rustfmt::skip]
/// Mouse layer
const L2: LayerMap = [
    [ _____ , F1    , F2    , F3    , F4    , F5    , _____ , /**/ _____ , F6    , F7    , F8    , F9    , F10   , F11   ],
    [ _____ , _____ , INSERT, HOME  , PGUP  , _____ , _____ , /**/ _____ , LEFT  , DOWN  , UP    , RIGHT , _____ , F12   ],
    [ _____ , _____ , DELETE, END   , PGDN  , _____ , _____ , /**/ _____ , _____ , M_L   ,MO_SCRL, M_R   , _____ , VOLUP ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , M_BCK , M_MID , M_FWD , _____ , VOLDN ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ DELETE, _____ , _____ , _____ , _____ , PRTSC , _____ ],
];

#[rustfmt::skip]
const L3: LayerMap = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , KP7   , KP8   , KP9   , _____ , _____ , /**/ _____ , SF(D1), SF(D2), SF(D3), SF(D4), SF(D5), _____ ],
    [ _____ , _____ , KP4   , KP5   , KP6   , _____ , _____ , /**/ _____ , SF(D6), SF(D7), SF(D8), SF(D9), SF(D0), _____ ],
    [ _____ , _____ , KP1   , KP2   , KP3   , _____ , _____ , /**/ _____ , QUOTE,SF(QUOTE),EQUAL,SF(EQUAL), _____ , _____ ],
    [ _____ , _____ , KP0   , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

#[rustfmt::skip]
const L4: LayerMap = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const KEYMAP: [Layer; CONFIG.layer_count] = [
    Layer {
        map: L0,
        arrowball: false,
    },
    Layer {
        map: L1,
        arrowball: false,
    },
    Layer {
        map: L2,
        arrowball: false,
    },
    Layer {
        map: L3,
        arrowball: true,
    },
    Layer {
        map: L4,
        arrowball: true,
    },
];