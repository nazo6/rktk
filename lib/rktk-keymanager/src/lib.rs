#![cfg_attr(not(test), no_std)]

//! # rktk-keymanager
//! A library for managing key state and keymaps for self-made keyboards.

use keycode::KeyDef;

pub mod keycode;
pub mod state;

pub struct Layer<const ROW: usize, const COL: usize> {
    pub map: [[KeyDef; COL]; ROW],
    pub arrowmouse: bool,
}

pub type LayerMap<const ROW: usize, const COL: usize> = [[KeyDef; COL]; ROW];
