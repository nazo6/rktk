//! Keymap related configs.

use super::CONST_CONFIG;

/// Re-exports of raw [`rktk_keymanager`] types.
///
/// Use parent module's type if available.
pub mod keymanager {
    pub use rktk_keymanager::keycode;
    pub use rktk_keymanager::keymap;
}

pub mod rktk_keys {
    use rktk_keymanager::keycode::prelude::*;

    macro_rules! gen_rktk_keys {
        ($($name:ident = $value:literal),* $(,)?) => {
            /// Custom key id of rktk-keymanager which is specific to rktk.
            /// It must be used with `Custom1` variant of [`rktk_keymanager::keycode::KeyCode`]
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
}

pub mod prelude {
    pub use super::rktk_keys::*;
    pub use rktk_keymanager::keycode::prelude::*;
}

pub type Keymap = rktk_keymanager::keymap::Keymap<
    { CONST_CONFIG.key_manager.layer_count as usize },
    { CONST_CONFIG.keyboard.rows as usize },
    { CONST_CONFIG.keyboard.cols as usize },
    { CONST_CONFIG.keyboard.encoder_count as usize },
    { CONST_CONFIG.key_manager.tap_dance_max_definitions },
    { CONST_CONFIG.key_manager.tap_dance_max_repeats },
    { CONST_CONFIG.key_manager.combo_key_max_definitions },
    { CONST_CONFIG.key_manager.combo_key_max_sources },
>;

pub type Layer = rktk_keymanager::keymap::Layer<
    { CONST_CONFIG.keyboard.rows as usize },
    { CONST_CONFIG.keyboard.cols as usize },
    { CONST_CONFIG.keyboard.encoder_count as usize },
>;

pub type LayerKeymap = rktk_keymanager::keymap::LayerKeymap<
    { CONST_CONFIG.keyboard.rows as usize },
    { CONST_CONFIG.keyboard.cols as usize },
>;
