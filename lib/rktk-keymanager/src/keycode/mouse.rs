//! Mouse button codes.

use bitflags::bitflags;
use macro_rules_attribute::apply;

use crate::macros::{common_derive, impl_display_bitflags, normal};

#[apply(common_derive)]
#[derive(Copy)]
pub struct Mouse(u8);

bitflags! {
    impl Mouse: u8 {
        const Left = 0b0000_0001;
        const Right = 0b0000_0010;
        const Middle = 0b0000_0100;
        const Back = 0b0000_1000;
        const Forward = 0b0001_0000;
    }
}

impl_display_bitflags!(Mouse);

normal!(M_L, Mouse, Left);
normal!(M_R, Mouse, Right);
normal!(M_MID, Mouse, Middle);
normal!(M_BCK, Mouse, Back);
normal!(M_FWD, Mouse, Forward);
