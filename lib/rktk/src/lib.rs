#![doc = include_str!("../README.md")]
//!
//! The main entry point is [`task::start`]. See the documentation for more details.

#![no_std]

pub mod config;
#[allow(async_fn_in_trait)]
pub mod interface;
pub mod panicking;
pub mod task;
mod utils;

use config::static_config::CONFIG;
pub use rktk_keymanager as keymanager;

pub type Layer = keymanager::Layer<{ CONFIG.rows }, { CONFIG.cols }>;
pub type LayerMap = keymanager::LayerMap<{ CONFIG.rows }, { CONFIG.cols }>;
