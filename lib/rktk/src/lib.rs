//! # rktk
//! ## Overview
//! `rktk` is a framework to build keyboard firmware. Using rktk, you can easily make feature-rich
//! highly customizable keyboard firmware.
//!
//! ## `rktk` crate
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
//! Basically, by passing [`drivers::Drivers`], [`hooks::Hooks`] and [`config::keymap::Keymap`] to [`task::start`], you can start the keyboard firmware.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//!
//! ### `alloc` feature
//! Embassy has the limitation that tasks with generics cannot be spawned.
//! For this reason, rktk, which makes heavy use of generics, uses the `join` method of embassy-sync to execute all tasks instead of spawning them.
//! However, this may be inferior to using spawning in terms of performance and power consumption.
//!
//! So if we enable `alloc` feature and provide an allocator, we can remove this limitation by spawning tasks in the heap.
//!
//! ## Note about statically configured value
//! You may see hard-coded values is used in some places (ex: [`config::keymap::Keymap`]).
//! These types are actually not hardcoded, but are configurable using json file.
//! Just a random value is provided because it is required to generate docs.
//!
//! For more detail, see [`config`].
//!
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod config;
pub mod dongle_task;
pub mod drivers;
pub mod hooks;
pub mod task;
pub mod utils;

#[doc(hidden)]
pub mod reexports {
    pub use heapless;
    pub use static_cell;
}
