//! Media keys.

use macro_rules_attribute::apply;

use crate::macros::{common_derive, impl_display, normal, with_consts};

/// Represents `media key` which is used for media control.
///
/// These keys are sent using a different descriptor than normal keys.
#[apply(with_consts)]
#[apply(common_derive)]
#[derive(Copy, strum::EnumIter, strum::IntoStaticStr)]
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

impl_display!(Media);

normal!(VOLUP, Media, VolumeIncrement);
normal!(VOLDN, Media, VolumeDecrement);
