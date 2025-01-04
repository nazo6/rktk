use crate::{
    interface::{state::config::MouseConfig, Output},
    keycode::KeyCode,
};

use super::{key_resolver::EventType, shared::SharedState, StateReport};

mod keyboard;
mod layer;
mod media_keyboard;
mod mouse;
mod transparent;

pub struct GlobalManagerState {
    mouse: mouse::MouseState,
    keyboard: keyboard::KeyboardState,
    media_keyboard: media_keyboard::MediaKeyboardState,
    transparent: transparent::TransparentState,
}

impl GlobalManagerState {
    pub fn new(mouse_config: MouseConfig, initial_output: Output) -> Self {
        GlobalManagerState {
            mouse: mouse::MouseState::new(mouse_config),
            keyboard: keyboard::KeyboardState::new(),
            media_keyboard: media_keyboard::MediaKeyboardState::new(),
            transparent: transparent::TransparentState::new(initial_output),
        }
    }
}

#[derive(Debug, Default)]
pub struct SharedLocalManagerState {
    pub normal_key_pressed: bool,
    pub keycodes: heapless::FnvIndexSet<u8, 8>,
}

pub struct LocalManagerState {
    mouse: mouse::MouseLocalState,
    keyboard: keyboard::KeyboardLocalState,
    media_keyboard: media_keyboard::MediaKeyboardLocalState,
    transparent: transparent::TransparentLocalState,
    shared: SharedLocalManagerState,
}

impl LocalManagerState {
    pub fn new(global_state: &GlobalManagerState) -> Self {
        LocalManagerState {
            mouse: mouse::MouseLocalState::new(),
            keyboard: keyboard::KeyboardLocalState::new(),
            media_keyboard: media_keyboard::MediaKeyboardLocalState::new(),
            transparent: transparent::TransparentLocalState::new(&global_state.transparent),
            shared: SharedLocalManagerState::default(),
        }
    }

    pub fn process_keycode<const LAYER: usize>(
        &mut self,
        layer_state: &mut [bool; LAYER],
        global_state: &mut GlobalManagerState,
        keycode: &KeyCode,
        event: EventType,
    ) {
        layer::layer_event_process(layer_state, keycode, event);
        self.mouse
            .process_event(&mut global_state.mouse, keycode, event);
        self.keyboard
            .process_event(&mut self.shared, keycode, event);
        self.media_keyboard.process_event(keycode);
        self.transparent.process_event(keycode, event);
    }

    pub fn process_mouse_event(&mut self, movement: (i8, i8)) {
        self.mouse.process_mouse_event(movement);
    }

    pub fn report<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
        const TAP_DANCE_MAX_DEFINITIONS: usize,
        const TAP_DANCE_MAX_REPEATS: usize,
        const COMBO_KEY_MAX_DEFINITIONS: usize,
        const COMBO_KEY_MAX_SOURCES: usize,
    >(
        mut self,
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
        global_state: &mut GlobalManagerState,
    ) -> StateReport {
        self.mouse
            .loop_end(shared_state, &mut self.shared, &mut global_state.mouse, 0);

        StateReport {
            keyboard_report: self
                .keyboard
                .report(&self.shared, &mut global_state.keyboard),
            mouse_report: self.mouse.report(&mut global_state.mouse),
            media_keyboard_report: self.media_keyboard.report(&mut global_state.media_keyboard),
            transparent_report: self.transparent.report(&mut global_state.transparent),
            highest_layer: shared_state.highest_layer() as u8,
        }
    }
}
