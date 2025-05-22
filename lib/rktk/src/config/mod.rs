//! Rktk configuration management.
//!
//! Constant(static) configuration of the firmware.
//!
//! These values are read from a json file set in the environment variable `RKTK_CONFIG_PATH`
//! and set at build time.
//!
//! It is convenient to set environment variables in `.cargo/config.toml` as follows.
//! ```toml
//! [env]
//! RKTK_CONFIG_PATH = { value = "rktk.json", relative = true }
//! ```
//! See the examples folder for an example of this json.
//!
//! For each configuration, see the [`schema`] module.

use crate::task::display::default_display::DefaultDisplayConfig;

include!(concat!(env!("OUT_DIR"), "/config.rs"));

pub mod keymap;
pub mod storage;

pub struct RktkOpts<D: crate::task::display::DisplayConfig> {
    pub keymap: &'static keymap::Keymap,
    pub config: &'static schema::DynamicConfig,
    pub display: D,
    pub hand: Option<crate::interface::Hand>,
}

pub fn new_rktk_opts(
    keymap: &'static keymap::Keymap,
    hand: Option<crate::interface::Hand>,
) -> RktkOpts<DefaultDisplayConfig> {
    RktkOpts {
        keymap,
        config: &DYNAMIC_CONFIG_FROM_FILE,
        display: DefaultDisplayConfig,
        hand,
    }
}
