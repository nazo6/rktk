use crate::time::{Duration, Instant};

use crate::keymap::Keymap;

pub(super) type LayerActive<const LAYER: usize> = [bool; LAYER];

pub(super) struct SharedState<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
> {
    pub keymap: Keymap<
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >,
    pub layer_active: LayerActive<LAYER>,
    pub now: Instant,
    pub locked: bool,
}

impl<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
>
    SharedState<
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >
{
    pub fn new(
        keymap: Keymap<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
    ) -> Self {
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
    fn is_arrow_mouse_layer(&self, layer: usize) -> bool;
    fn now(&self) -> Instant;
    fn set_layer_active(&mut self, layer: usize, active: bool);
}

impl<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
> KeycodeUpdateContext for SharedState<
    LAYER,
    ROW,
    COL,
    ENCODER_COUNT,
    TAP_DANCE_MAX_DEFINITIONS,
    TAP_DANCE_MAX_REPEATS,
    COMBO_KEY_MAX_DEFINITIONS,
    COMBO_KEY_MAX_SOURCES,
> {
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

impl<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
> MouseEndContext for SharedState<
    LAYER,
    ROW,
    COL,
    ENCODER_COUNT,
    TAP_DANCE_MAX_DEFINITIONS,
    TAP_DANCE_MAX_REPEATS,
    COMBO_KEY_MAX_DEFINITIONS,
    COMBO_KEY_MAX_SOURCES,
> {
    fn is_arrow_mouse_layer(&self, layer: usize) -> bool {
        self.keymap.layers.get(layer).map(|l| l.arrow_mouse).unwrap_or(false)
    }

    fn now(&self) -> Instant {
        self.now
    }

    fn set_layer_active(&mut self, layer: usize, active: bool) {
        if let Some(slot) = self.layer_active.get_mut(layer) {
            *slot = active;
        }
    }
}
