#![doc = include_str!("../README.md")]
//! This crate provides the rktk drivers for the RP2040 chip.
//!
//! Many drivers is in this crate are just convenient wrappers over [`rktk_drivers_common`], but
//! implements some original drivers like [`split::pio_half_duplex`].
#![no_std]

pub mod display;
pub mod flash;
pub mod keyscan;
pub mod mouse;
pub mod rgb;
pub mod split;
pub mod system;
pub mod usb;
