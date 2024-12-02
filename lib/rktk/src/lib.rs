//! # rktk
//! ## Overview
//! `rktk` is a framework to build keyboard firmware. Using rktk, you can easily make feature-rich
//! highly customizable keyboard firmware.
//!
//! For full list of supported features, see [RKTK project README](https://github.com/nazo6/rktk).
//!
//! # `rktk` crate
//!
//! This `rktk` crate is the main crate of the project. It contains the main logic of the
//! keyboard firmware and does not depend on any specific hardware.
//!
//! This crate consists of the following modules:
//! - [`task`]: The main task that runs the keyboard firmware.
//! - [`drivers`]: Drivers that are used by the task.
//! - [`hooks`]: Hooks that can be used to customize the behavior of the application.
//! - [`config`]: Configuration of the keyboard.
//!
//! Basically, by passing [`drivers::Drivers`], [`hooks::Hooks`] and [`keymap_config::KeyConfig`] to [`task::start`], you can start the keyboard firmware.
//!
//! # Note about static configured value.
//! You may see some type has hard-coded const generics (ex: [`keymap_config::Keymap`]). These
//! These types are not actually hardcoded, but are configurable using the `rktk.json` file.
//! Just a random value is provided because it is required to generate docs.

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
        pub keymap: keymanager::keymap::Keymap<
            { RKTK_CONFIG.layer_count as usize },
            { KEYBOARD.rows as usize },
            { KEYBOARD.cols as usize },
            { KEYBOARD.encoder_count as usize },
        >,
        pub tap_dance: [Option<TapDanceConfig>; 8],
    }

    pub type Keymap = keymanager::keymap::Keymap<
        { RKTK_CONFIG.layer_count as usize },
        { KEYBOARD.rows as usize },
        { KEYBOARD.cols as usize },
        { KEYBOARD.encoder_count as usize },
    >;

    pub type Layer =
        keymanager::keymap::Layer<{ KEYBOARD.rows as usize }, { KEYBOARD.cols as usize }>;

    pub type LayerMap =
        keymanager::keymap::LayerMap<{ KEYBOARD.rows as usize }, { KEYBOARD.cols as usize }>;
}
