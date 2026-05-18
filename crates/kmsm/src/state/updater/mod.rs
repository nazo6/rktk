use crate::{
    interface::state::{
        config::MouseConfig,
        output_event::{EventType, OutputEvent},
    },
    keycode::{KeyCode, special::Special},
};

use super::shared::{KeycodeUpdateContext, MouseEndContext};

pub(crate) mod layer;
pub(crate) mod mouse;

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
    pub fn update_by_keycode(
        &mut self,
        kc: &KeyCode,
        ev: EventType,
        shared_state: &mut impl KeycodeUpdateContext,
        mut cb: impl FnMut(OutputEvent),
    ) {
        if ev == EventType::Pressed && *kc == KeyCode::Special(Special::LockTg) {
            shared_state.toggle_locked();
        }
        if shared_state.is_locked() && ev != EventType::Released {
            return;
        }

        shared_state.update_layer(kc, ev);
        self.mouse.update_by_keycode(kc, ev, &mut cb);

        let output_event = OutputEvent::KeyCode((*kc, ev));
        cb(output_event);
    }

    pub fn update_by_mouse_move(&mut self, mv: (i8, i8), _cb: impl FnMut(OutputEvent)) {
        self.mouse.update_by_mouse_move(mv);
    }

    pub fn end(
        self,
        highest_layer: usize,
        shared_state: &mut impl MouseEndContext,
        cb: impl FnMut(OutputEvent),
    ) {
        self.mouse.end(highest_layer, shared_state, cb);
    }
}
