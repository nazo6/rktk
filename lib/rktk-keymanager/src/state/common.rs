use crate::time::Instant;

use crate::{keycode::KeyAction, Keymap};

pub(super) struct CommonState<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
> {
    pub keymap: Keymap<LAYER, ROW, COL, ENCODER_COUNT>,
    pub layer_active: [bool; LAYER],
}

impl<const LAYER: usize, const ROW: usize, const COL: usize, const ENCODER_COUNT: usize>
    CommonState<LAYER, ROW, COL, ENCODER_COUNT>
{
    pub fn new(keymap: Keymap<LAYER, ROW, COL, ENCODER_COUNT>) -> Self {
        Self {
            keymap,
            layer_active: [false; LAYER],
        }
    }

    pub fn highest_layer(&self) -> usize {
        self.layer_active.iter().rposition(|&x| x).unwrap_or(0)
    }

    /// Get the key action for the given key position and if it is inherited, resolve the inherited key action
    pub fn get_inherited_keyaction(&self, row: u8, col: u8, layer: usize) -> Option<KeyAction> {
        if row >= ROW as u8 || col >= COL as u8 {
            return None;
        }

        for layer in (0..=layer).rev() {
            let ka = &self.keymap.layers[layer].map[row as usize][col as usize];
            if let KeyAction::Inherit = ka {
                continue;
            } else {
                return Some(*ka);
            }
        }

        None
    }
}

pub(super) struct CommonLocalState {
    pub normal_key_pressed: bool,
    pub keycodes: crate::Vec<u8, 6>,
    pub now: Instant,
}

impl CommonLocalState {
    pub fn new(now: Instant) -> Self {
        Self {
            normal_key_pressed: false,
            keycodes: crate::Vec::new(),
            now,
        }
    }
}
