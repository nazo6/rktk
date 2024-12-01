#![doc = include_str!("../README.md")]
//!
//! This crate consists of the following modules:
//! - [`task`]: The main task that runs the keyboard firmware.
//! - [`drivers`]: Drivers that are used by the task.
//! - [`hooks`]: Hooks that can be used to customize the behavior of the application.
//! - [`config`]: Configuration of the keyboard.
//!
//! Basically, by passing [`drivers::Drivers`], [`hooks::Hooks`] and [`keymap_config::KeyConfig`] to [`task::start`], you can start the keyboard firmware.

#![no_std]

pub mod config;
pub mod drivers;
pub mod hooks;
pub mod task;
pub mod utils;

#[doc(hidden)]
pub mod reexports {
    pub use heapless;
}

pub use rktk_keymanager as keymanager;

/// Keymap configuration types.
pub mod keymap_config {
    use crate::config::static_config::{KEYBOARD, RKTK_CONFIG};
    use crate::keymanager;
    use rktk_keymanager::state::config::TapDanceConfig;

    pub struct KeyConfig {
        pub keymap: keymanager::Keymap<
            { RKTK_CONFIG.layer_count as usize },
            { KEYBOARD.rows as usize },
            { KEYBOARD.cols as usize },
            { KEYBOARD.encoder_count as usize },
        >,
        pub tap_dance: [Option<TapDanceConfig>; 8],
    }

    pub type Keymap = keymanager::Keymap<
        { RKTK_CONFIG.layer_count as usize },
        { KEYBOARD.rows as usize },
        { KEYBOARD.cols as usize },
        { KEYBOARD.encoder_count as usize },
    >;

    pub type Layer = keymanager::Layer<{ KEYBOARD.rows as usize }, { KEYBOARD.cols as usize }>;

    pub type LayerMap =
        keymanager::LayerMap<{ KEYBOARD.rows as usize }, { KEYBOARD.cols as usize }>;
}
