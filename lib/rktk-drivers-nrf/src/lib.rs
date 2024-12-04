#![doc = include_str!("../README.md")]
//!
//! This crate provides the rktk drivers for the nRF series microcontrollers. Currently, only nRF52840
//! is enabled, but it should be easy to add support for other nRF series microcontrollers.
//!
//! Many drivers is in this crate are just convenient wrappers over [`rktk_drivers_common`], but
//! implements some original drivers like BLE and Uart split driver.
//!
//! NOTE: This crate uses unreleased version of `nrf-softdevice` and such dependency is not accepted by crates.io.
//! So if you want to use `softdevice` feature (needed for BLE), use git version of this crate.

#![no_std]
#![cfg_attr(feature = "nightly", feature(impl_trait_in_assoc_type))]

pub mod display;
pub mod keyscan;
pub mod mouse;
pub mod rgb;
#[cfg(feature = "softdevice")]
pub mod softdevice;
pub mod split;
pub mod system;
