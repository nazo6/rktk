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
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
#![no_std]
#![cfg_attr(doc, feature(doc_cfg))]

pub mod display;
#[cfg(feature = "esb")]
pub mod esb;
pub mod keyscan;
pub mod mouse;
pub mod rgb;
#[cfg(feature = "sdc")]
pub mod sdc;
#[cfg(feature = "softdevice")]
pub mod softdevice;
pub mod split;
pub mod system;
