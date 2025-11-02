use crate::{
    interface::state::{
        config::MouseConfig,
        output_event::{EventType, OutputEvent},
    },
    keycode::{KeyCode, special::Special},
};

use super::shared::SharedState;

mod layer;
mod mouse;

pub struct UpdaterState {
    mouse: mouse::MouseState,
}

impl UpdaterState {
    pub fn new(mouse_config: MouseConfig) -> Self {
        Self {
            mouse: mouse::MouseState::new(mouse_config),
        }
    }

    pub fn start_update<'a>(&'a mut self) -> Updater<'a> {
        Updater {
            mouse: self.mouse.start_update(),
        }
    }
}

pub struct Updater<'a> {
    mouse: mouse::MouseUpdater<'a>,
}

impl Updater<'_> {
    pub fn update_by_keycode<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
        const TAP_DANCE_MAX_DEFINITIONS: usize,
        const TAP_DANCE_MAX_REPEATS: usize,
        const COMBO_KEY_MAX_DEFINITIONS: usize,
        const COMBO_KEY_MAX_SOURCES: usize,
    >(
        &mut self,
        kc: &KeyCode,
        ev: EventType,
        shared_state: &mut SharedState<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
        mut cb: impl FnMut(OutputEvent),
    ) {
        if ev == EventType::Pressed && *kc == KeyCode::Special(Special::LockTg) {
            shared_state.locked = !shared_state.locked;
        }
        if shared_state.locked && ev != EventType::Released {
            return;
        }

        layer::update_layer_by_keycode(&mut shared_state.layer_active, kc, ev);
        self.mouse.update_by_keycode(kc, ev, &mut cb);

        let output_event = OutputEvent::KeyCode((*kc, ev));
        cb(output_event);
    }

    pub fn update_by_mouse_move(&mut self, mv: (i8, i8), _cb: impl FnMut(OutputEvent)) {
        self.mouse.update_by_mouse_move(mv);
    }

    pub fn end<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
        const TAP_DANCE_MAX_DEFINITIONS: usize,
        const TAP_DANCE_MAX_REPEATS: usize,
        const COMBO_KEY_MAX_DEFINITIONS: usize,
        const COMBO_KEY_MAX_SOURCES: usize,
    >(
        self,
        highest_layer: usize,
        shared_state: &mut SharedState<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
        cb: impl FnMut(OutputEvent),
    ) {
        self.mouse.end(highest_layer, shared_state, cb);
    }
}
