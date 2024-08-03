#![doc = include_str!("../README.md")]
//!
//! The main entry point is [`task::start`]. See the documentation for more details.

#![no_std]

pub mod config;
#[allow(async_fn_in_trait)]
pub mod interface;
pub mod keycode;
mod state;
pub mod task;
mod utils;
