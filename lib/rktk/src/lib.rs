#![doc = include_str!("../README.md")]
//!
//! The main entry point is [`task::start`]. See the documentation for more details.

#![no_std]

pub mod config;
#[allow(async_fn_in_trait)]
pub mod interface;
pub mod task;
mod utils;

#[doc(hidden)]
pub mod reexports {
    pub use heapless;
}

use config::static_config::CONFIG;
pub use log;
pub use rktk_keymanager as keymanager;
use rktk_keymanager::state::config::TapDanceConfig;

pub type Layer = keymanager::Layer<{ CONFIG.rows as usize }, { CONFIG.cols as usize }>;
pub type LayerMap = keymanager::LayerMap<{ CONFIG.rows as usize }, { CONFIG.cols as usize }>;
pub type Keymap = keymanager::Keymap<
    { CONFIG.layer_count as usize },
    { CONFIG.rows as usize },
    { CONFIG.cols as usize },
>;
pub struct KeyConfig {
    pub keymap: keymanager::Keymap<
        { CONFIG.layer_count as usize },
        { CONFIG.rows as usize },
        { CONFIG.cols as usize },
    >,
    pub tap_dance: [Option<TapDanceConfig>; 8],
}
