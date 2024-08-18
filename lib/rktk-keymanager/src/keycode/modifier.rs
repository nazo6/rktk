use super::{KeyAction, KeyCode};
use bitflags::bitflags;

use super::macros::normal;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Modifier(u8);

bitflags! {
    impl Modifier: u8 {
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

normal!(L_CTRL, Modifier, LCtrl);
normal!(L_SHFT, Modifier, LShft);
normal!(L_ALT, Modifier, LAlt);
normal!(L_GUI, Modifier, LGui);
normal!(R_CTRL, Modifier, RCtrl);
normal!(R_SHFT, Modifier, RShft);
normal!(R_ALT, Modifier, RAlt);
normal!(R_GUI, Modifier, RGui);
