#![no_std]

use kmsm::keycode::prelude::*;

macro_rules! gen_rktk_keys {
    ($($name:ident = $value:literal),* $(,)?) => {
        /// Custom key id of kmsm which is specific to rktk.
        /// It must be used with `Custom1` variant of [`kmsm::keycode::KeyCode`]
        ///
        /// Custom2 and Custom3 can be used by user.
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter, strum::IntoStaticStr, strum::FromRepr,
        )]
        #[repr(u8)]
        pub enum RktkKeys {
            $(
                $name = $value,
            )*
        }

        paste::paste! {
            $(pub const [<$name:snake:upper>] : KeyAction = KeyAction::Normal(KeyCode::Custom1($value));)*
        }
    };
    }

gen_rktk_keys! {
    FlashClear = 0,
    OutputBle = 1,
    OutputUsb = 2,
    BleBondClear = 3,
    Bootloader = 4,
    PowerOff = 5,
    RgbOff = 6,
    RgbBrightnessUp = 7,
    RgbBrightnessDown = 8,
    RgbPatternRainbow = 9,
}

use core::fmt::{self, Display, Formatter};
impl Display for RktkKeys {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{s}")
    }
}
