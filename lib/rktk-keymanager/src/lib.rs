#![doc = include_str!("../README.md")]
//!
//! This crate consists of the following modules:
//! - [`keycode`]: Keycode definitions
//! - [`keymap`]: Keymap definition
//! - [`state`]: State management
//!
//! To know how to define keymap, see `keycode` and `keymap` modules.

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![cfg_attr(doc, feature(doc_auto_cfg))]
#![allow(non_snake_case)]

pub mod interface;
pub mod keycode;
pub mod keymap;
mod macros;
mod time;

#[cfg(any(test, feature = "state"))]
pub mod state;
