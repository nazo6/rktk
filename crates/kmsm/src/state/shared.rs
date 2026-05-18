use crate::time::{Duration, Instant};

pub(super) type LayerActive<const LAYER: usize> = [bool; LAYER];

pub(super) struct SharedState<K, const LAYER: usize> {
    pub keymap: K,
    pub layer_active: LayerActive<LAYER>,
    pub now: Instant,
    pub locked: bool,
}

impl<K, const LAYER: usize> SharedState<K, LAYER> {
    pub fn new(keymap: K) -> Self {
        Self {
            keymap,
            layer_active: [false; LAYER],
            now: Instant::from_start(Duration::from_millis(0)),
            locked: false,
        }
    }

    pub fn highest_layer(&self) -> usize {
        self.layer_active.iter().rposition(|&x| x).unwrap_or(0)
    }
}

pub trait KeycodeUpdateContext {
    fn is_locked(&self) -> bool;
    fn toggle_locked(&mut self);
    fn update_layer(&mut self, kc: &crate::keycode::KeyCode, ev: crate::interface::state::output_event::EventType);
}

pub trait MouseEndContext {
    fn now(&self) -> Instant;
    fn set_layer_active(&mut self, layer: usize, active: bool);
}

impl<K, const LAYER: usize> KeycodeUpdateContext for SharedState<K, LAYER> {
    fn is_locked(&self) -> bool {
        self.locked
    }

    fn toggle_locked(&mut self) {
        self.locked = !self.locked;
    }

    fn update_layer(&mut self, kc: &crate::keycode::KeyCode, ev: crate::interface::state::output_event::EventType) {
        crate::state::updater::layer::update_layer_by_keycode(&mut self.layer_active, kc, ev);
    }
}

impl<K, const LAYER: usize> MouseEndContext for SharedState<K, LAYER> {
    fn now(&self) -> Instant {
        self.now
    }

    fn set_layer_active(&mut self, layer: usize, active: bool) {
        if let Some(slot) = self.layer_active.get_mut(layer) {
            *slot = active;
        }
    }
}
