#![doc = include_str!("../../../README.md")]
//!
//! # `rktk` crate
//! `rktk` crate is hardware-agnostic library to build keyboard firmware. It receives the driver
//! and executes the problem.
//!
//! The main entry point is [`task::start`]. See the documentation for more details.

#![no_std]
pub mod config;
pub mod constant;
#[allow(async_fn_in_trait)]
pub mod interface;
pub mod keycode;
mod state;
pub mod task;
mod utils;
