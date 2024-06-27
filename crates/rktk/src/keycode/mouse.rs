use bitflags::bitflags;

use super::macros::bit_normal;
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

bit_normal!(M_L, Mouse, Left);
bit_normal!(M_R, Mouse, Right);
bit_normal!(M_MID, Mouse, Middle);
bit_normal!(M_BCK, Mouse, Back);
bit_normal!(M_FWD, Mouse, Forward);
