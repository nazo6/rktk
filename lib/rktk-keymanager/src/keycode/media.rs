use macro_rules_attribute::apply;

use crate::macros::{common_derive, normal, with_consts};

#[apply(with_consts)]
#[apply(common_derive)]
#[derive(Copy)]
pub enum Media {
    Zero = 0x00,
    Play = 0xB0,
    Pause = 0xB1,
    Record = 0xB2,
    NextTrack = 0xB5,
    PrevTrack = 0xB6,
    Stop = 0xB7,
    RandomPlay = 0xB9,
    Repeat = 0xBC,
    PlayPause = 0xCD,
    Mute = 0xE2,
    VolumeIncrement = 0xE9,
    VolumeDecrement = 0xEA,
    Reserved = 0xEB,
}

normal!(VOLUP, Media, VolumeIncrement);
normal!(VOLDN, Media, VolumeDecrement);
