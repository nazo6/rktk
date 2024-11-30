#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![allow(non_snake_case)]

//! # rktk-keymanager
//! A library for managing key state and keymaps for self-made keyboards.
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
type Vec<T, const N: usize> = alloc::vec::Vec<T>;

#[cfg(all(feature = "heapless", not(feature = "alloc")))]
type Vec<T, const N: usize> = heapless::Vec<T, N>;

pub mod keycode;
mod macros;
#[cfg(any(test, feature = "state"))]
pub mod state;
mod time;
#[cfg(not(any(test, feature = "state")))]
pub mod state {
    pub mod config;
}

use keycode::{KeyAction, KeyCode};

#[cfg_attr(feature = "serde", serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Layer<const ROW: usize, const COL: usize> {
    // NOTE: This is workaround for issue that serde_as cannot be used with cfg-attr.
    // ref: https://github.com/jonasbb/serde_with/issues/355
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<[[serde_with::Same; COL]; ROW]>")
    )]
    pub map: LayerMap<ROW, COL>,
    pub arrowmouse: bool,
}

impl<const ROW: usize, const COL: usize> Default for Layer<ROW, COL> {
    fn default() -> Self {
        Self {
            map: [[KeyAction::default(); COL]; ROW],
            arrowmouse: false,
        }
    }
}

pub type LayerMap<const ROW: usize, const COL: usize> = [[KeyAction; COL]; ROW];

#[derive(Clone)]
pub struct Keymap<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
> {
    pub layers: [Layer<ROW, COL>; LAYER],
    pub encoder_keys: [(KeyCode, KeyCode); ENCODER_COUNT],
}
