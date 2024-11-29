#![doc = include_str!("../README.md")]
//!
//! The main entry point is [`task::start`]. See the documentation for more details.

#![no_std]

pub mod config;
pub mod hooks;
pub mod interface;
pub mod task;
mod utils;
use log;

#[doc(hidden)]
pub mod reexports {
    pub use heapless;
}

use config::static_config::{KEYBOARD, RKTK_CONFIG};
pub use rktk_keymanager as keymanager;
use rktk_keymanager::state::config::TapDanceConfig;

pub type Layer = keymanager::Layer<{ KEYBOARD.rows as usize }, { KEYBOARD.cols as usize }>;
pub type LayerMap = keymanager::LayerMap<{ KEYBOARD.rows as usize }, { KEYBOARD.cols as usize }>;
pub type Keymap = keymanager::Keymap<
    { RKTK_CONFIG.layer_count as usize },
    { KEYBOARD.rows as usize },
    { KEYBOARD.cols as usize },
    { KEYBOARD.encoder_count as usize },
>;
pub struct KeyConfig {
    pub keymap: keymanager::Keymap<
        { RKTK_CONFIG.layer_count as usize },
        { KEYBOARD.rows as usize },
        { KEYBOARD.cols as usize },
        { KEYBOARD.encoder_count as usize },
    >,
    pub tap_dance: [Option<TapDanceConfig>; 8],
}
