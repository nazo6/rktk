use embassy_time::Instant;

use crate::{
    keycode::{KeyAction, KeyDef},
    Layer,
};

pub(super) struct CommonState<const LAYER: usize, const ROW: usize, const COL: usize> {
    pub layers: [Layer<ROW, COL>; LAYER],
    pub layer_active: [bool; LAYER],
}

impl<const LAYER: usize, const ROW: usize, const COL: usize> CommonState<LAYER, ROW, COL> {
    pub fn new(layers: [Layer<ROW, COL>; LAYER]) -> Self {
        Self {
            layers,
            layer_active: [false; LAYER],
        }
    }

    pub fn highest_layer(&self) -> usize {
        self.layer_active.iter().rposition(|&x| x).unwrap_or(0)
    }

    pub fn get_keyaction(&self, row: u8, col: u8, layer: usize) -> Option<KeyAction> {
        if row >= ROW as u8 || col >= COL as u8 {
            return None;
        }

        for layer in (0..=layer).rev() {
            let key = &self.layers[layer].map[row as usize][col as usize];
            match key {
                KeyDef::None => return None,
                KeyDef::Inherit => continue,
                KeyDef::Key(action) => return Some(*action),
            }
        }

        None
    }
}

pub(super) struct CommonLocalState {
    pub normal_key_pressed: bool,
    pub keycodes: heapless::Vec<u8, 6>,
    pub now: Instant,
}

impl CommonLocalState {
    pub fn new(now: Instant) -> Self {
        Self {
            normal_key_pressed: false,
            keycodes: heapless::Vec::new(),
            now,
        }
    }
}
