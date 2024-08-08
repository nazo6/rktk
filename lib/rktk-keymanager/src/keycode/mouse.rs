use bitflags::bitflags;

use super::macros::normal;
use super::{KeyAction, KeyCode, KeyDef};

bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub struct Mouse: u8 {
        const Left = 0b0000_0001;
        const Right = 0b0000_0010;
        const Middle = 0b0000_0100;
        const Back = 0b0000_1000;
        const Forward = 0b0001_0000;
    }
}

normal!(M_L, Mouse, Left);
normal!(M_R, Mouse, Right);
normal!(M_MID, Mouse, Middle);
normal!(M_BCK, Mouse, Back);
normal!(M_FWD, Mouse, Forward);
