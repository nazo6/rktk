//! Rktk configuration management.

use crate::task::display::default_display::DefaultDisplayConfig;

pub mod constant;
pub mod keymap;
pub mod storage;

pub struct RktkOpts<D: crate::task::display::DisplayConfig = DefaultDisplayConfig> {
    pub keymap: &'static keymap::Keymap,
    pub config: &'static constant::schema::DynamicConfig,
    pub display: Option<D>,
    pub hand: Option<crate::interface::Hand>,
}
