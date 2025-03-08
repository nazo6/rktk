//! Keymap related configs.

/// Re-exports of raw [`rktk_keymanager`] types.
///
/// Use parent module's type if available.
pub mod keymanager {
    pub use rktk_keymanager::keycode;
    pub use rktk_keymanager::keymap;
}

use crate::config::constant::{KEYBOARD, KM_CONFIG, RKTK_CONFIG};

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
