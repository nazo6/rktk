#![cfg_attr(all(not(test), not(feature = "specta")), no_std)]

//! # rktk-keymanager
//! A library for managing key state and keymaps for self-made keyboards.

use keycode::KeyAction;

pub mod keycode;
#[cfg(any(test, feature = "state"))]
pub mod state;

#[derive(Clone, Debug)]
pub struct Layer<const ROW: usize, const COL: usize> {
    pub map: [[KeyAction; COL]; ROW],
    pub arrowmouse: bool,
}
pub type LayerMap<const ROW: usize, const COL: usize> = [[KeyAction; COL]; ROW];
pub type Keymap<const LAYER: usize, const ROW: usize, const COL: usize> = [Layer<ROW, COL>; LAYER];
