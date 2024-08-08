#![cfg_attr(not(test), no_std)]

use keycode::KeyDef;

pub mod keycode;
pub mod state;

pub struct Layer<const ROW: usize, const COL: usize> {
    pub map: [[KeyDef; COL]; ROW],
    pub arrowball: bool,
}

pub type LayerMap<const ROW: usize, const COL: usize> = [[KeyDef; COL]; ROW];
