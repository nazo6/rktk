use super::{KeyAction, KeyCode, KeyDef};
use bitflags::bitflags;

use super::macros::bit_normal;

bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub struct Modifier: u8 {
        const LCtrl = 0x01;
        const LShft = 0x02;
        const LAlt = 0x04;
        const LGui = 0x08;
        const RCtrl = 0x10;
        const RShft = 0x20;
        const RAlt = 0x40;
        const RGui = 0x80;
    }
}

bit_normal!(L_CTRL, Modifier, LCtrl);
bit_normal!(L_SHFT, Modifier, LShft);
bit_normal!(L_ALT, Modifier, LAlt);
bit_normal!(L_GUI, Modifier, LGui);
bit_normal!(R_CTRL, Modifier, RCtrl);
bit_normal!(R_SHFT, Modifier, RShft);
bit_normal!(R_ALT, Modifier, RAlt);
bit_normal!(R_GUI, Modifier, RGui);
