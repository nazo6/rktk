#![doc = include_str!("../README.md")]
//!
//! This crate consists of the following modules:
//! - [`keycode`]: Keycode definitions
//! - [`keymap`]: Keymap definition
//! - [`state`]: State management
//!
//! To know how to define keymap, see `keycode` and `keymap` modules.

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![allow(non_snake_case)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
type Vec<T, const N: usize> = alloc::vec::Vec<T>;

#[cfg(all(feature = "heapless", not(feature = "alloc")))]
type Vec<T, const N: usize> = heapless::Vec<T, N>;

pub mod keycode;
pub mod keymap;
mod macros;
mod time;

#[cfg(any(test, feature = "state"))]
pub mod state;
#[cfg(not(any(test, feature = "state")))]
pub mod state {
    pub mod config;
}
