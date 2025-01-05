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
//! Basically, by passing [`drivers::Drivers`], [`hooks::Hooks`] and [`keymap_config::Keymap`] to [`task::start`], you can start the keyboard firmware.
//!
//! ## Note about statically configured value
//! You may see hard-coded values is used in some places (ex: [`keymap_config::Keymap`]).
//! These types are actually not hardcoded, but are configurable using json file.
//! Just a random value is provided because it is required to generate docs.
//!
//! For more detail, see [`config::static_config`].

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
