// use core::array::from_fn;

use embassy_time::Instant;

use crate::{
    config::static_config::CONFIG,
    keycode::{KeyAction, KeyDef, Layer},
};

// use self::key::KeyState;

// mod key;

pub(super) struct CommonState {
    pub layers: [Layer; CONFIG.layer_count],
    pub layer_active: [bool; CONFIG.layer_count],
    // pub key_state: [[KeyState; CONFIG.cols * 2]; ROWS],
}

impl CommonState {
    pub fn new(layers: [Layer; CONFIG.layer_count]) -> Self {
        Self {
            layers,
            layer_active: [false; CONFIG.layer_count],
            // key_state: from_fn(|_| from_fn(|_| KeyState::default())),
        }
    }

    pub fn highest_layer(&self) -> usize {
        self.layer_active.iter().rposition(|&x| x).unwrap_or(0)
    }

    pub fn get_keycode(&self, row: u8, col: u8, layer: usize) -> Option<KeyAction> {
        if row >= (CONFIG.rows) as u8 || col >= (CONFIG.cols * 2) as u8 {
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
    pub prev_highest_layer: usize,
    pub normal_key_pressed: bool,
    pub keycodes: heapless::Vec<u8, 6>,
    pub now: Instant,
}

impl CommonLocalState {
    pub fn new(prev_highest_layer: usize) -> Self {
        Self {
            prev_highest_layer,
            normal_key_pressed: false,
            keycodes: heapless::Vec::new(),
            now: Instant::now(),
        }
    }
}
