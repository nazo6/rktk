//! Keymap related configs.
use crate::config::constant::{KEYBOARD, KM_CONFIG, RKTK_CONFIG};

/// Re-exports of raw [`rktk_keymanager`] types.
///
/// Use parent module's type if available.
pub mod keymanager {
    pub use rktk_keymanager::keycode;
    pub use rktk_keymanager::keymap;
}

pub mod rktk_keys {
    use rktk_keymanager::keycode::prelude::*;

    /// Custom key id of rktk-keymanager which is specific to rktk.
    /// It must be used with `Custom1` variant of [`rktk_keymanager::keycode::KeyCode`]
    ///
    /// Custom2 and Custom3 can be used by user.
    pub enum RktkKeys {
        FlashClear = 0,
        OutputBle = 1,
        OutputUsb = 2,
        BleBondClear = 3,
        Bootloader = 4,
        PowerOff = 5,
    }

    impl TryFrom<u8> for RktkKeys {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::FlashClear),
                1 => Ok(Self::OutputBle),
                2 => Ok(Self::OutputUsb),
                3 => Ok(Self::BleBondClear),
                4 => Ok(Self::Bootloader),
                5 => Ok(Self::PowerOff),
                _ => Err(()),
            }
        }
    }

    pub const FLASH_CLEAR: KeyAction =
        KeyAction::Normal(KeyCode::Custom1(RktkKeys::FlashClear as u8));
    pub const OUTPUT_BLE: KeyAction =
        KeyAction::Normal(KeyCode::Custom1(RktkKeys::OutputBle as u8));
    pub const OUTPUT_USB: KeyAction =
        KeyAction::Normal(KeyCode::Custom1(RktkKeys::OutputUsb as u8));
    pub const BLE_BOND_CLEAR: KeyAction =
        KeyAction::Normal(KeyCode::Custom1(RktkKeys::BleBondClear as u8));
    pub const BOOTLOADER: KeyAction =
        KeyAction::Normal(KeyCode::Custom1(RktkKeys::Bootloader as u8));
    pub const POWER_OFF: KeyAction = KeyAction::Normal(KeyCode::Custom1(RktkKeys::PowerOff as u8));
}

pub mod prelude {
    pub use super::rktk_keys::*;
    pub use rktk_keymanager::keycode::prelude::*;
}

pub type Keymap = rktk_keymanager::keymap::Keymap<
    { RKTK_CONFIG.layer_count as usize },
    { KEYBOARD.rows as usize },
    { KEYBOARD.cols as usize },
    { KEYBOARD.encoder_count as usize },
    { KM_CONFIG.constant.tap_dance_max_definitions },
    { KM_CONFIG.constant.tap_dance_max_repeats },
    { KM_CONFIG.constant.combo_key_max_definitions },
    { KM_CONFIG.constant.combo_key_max_sources },
>;

pub type Layer = rktk_keymanager::keymap::Layer<
    { KEYBOARD.rows as usize },
    { KEYBOARD.cols as usize },
    { KEYBOARD.encoder_count as usize },
>;

pub type LayerKeymap =
    rktk_keymanager::keymap::LayerKeymap<{ KEYBOARD.rows as usize }, { KEYBOARD.cols as usize }>;
