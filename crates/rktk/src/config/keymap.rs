//! Keymap related configs.

use super::CONST_CONFIG;

/// Re-exports of raw [`kmsm`] types.
///
/// Use parent module's type if available.
pub mod keymanager {
    pub use kmsm::keycode;
    pub use kmsm::keymap;
}

pub mod prelude {
    pub use kmsm::keycode::prelude::*;
    pub use kmsm_rktk::*;
}

pub type Keymap = kmsm::keymap::Keymap<
    { CONST_CONFIG.key_manager.layer_count as usize },
    { CONST_CONFIG.keyboard.rows as usize },
    { CONST_CONFIG.keyboard.cols as usize },
    { CONST_CONFIG.keyboard.encoder_count as usize },
    { CONST_CONFIG.key_manager.tap_dance_max_definitions },
    { CONST_CONFIG.key_manager.tap_dance_max_repeats },
    { CONST_CONFIG.key_manager.combo_key_max_definitions },
    { CONST_CONFIG.key_manager.combo_key_max_sources },
>;

pub type Layer = kmsm::keymap::Layer<
    { CONST_CONFIG.keyboard.rows as usize },
    { CONST_CONFIG.keyboard.cols as usize },
    { CONST_CONFIG.keyboard.encoder_count as usize },
>;

pub type LayerKeymap = kmsm::keymap::LayerKeymap<
    { CONST_CONFIG.keyboard.rows as usize },
    { CONST_CONFIG.keyboard.cols as usize },
>;
