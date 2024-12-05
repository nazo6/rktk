use crate::time::{Duration, Instant};

use crate::keymap::Keymap;

pub(super) type LayerActive<const LAYER: usize> = [bool; LAYER];

pub(super) struct SharedState<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
> {
    pub keymap: Keymap<LAYER, ROW, COL, ENCODER_COUNT>,
    pub layer_active: LayerActive<LAYER>,
    pub now: Instant,
}

impl<const LAYER: usize, const ROW: usize, const COL: usize, const ENCODER_COUNT: usize>
    SharedState<LAYER, ROW, COL, ENCODER_COUNT>
{
    pub fn new(keymap: Keymap<LAYER, ROW, COL, ENCODER_COUNT>) -> Self {
        Self {
            keymap,
            layer_active: [false; LAYER],
            now: Instant::from_start(Duration::from_millis(0)),
        }
    }

    pub fn highest_layer(&self) -> usize {
        self.layer_active.iter().rposition(|&x| x).unwrap_or(0)
    }
}
