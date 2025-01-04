use crate::{interface::state::event::KeyChangeEvent, keycode::KeyCode};

use super::EventType;

/// State management for Normal and Normal2 action
pub struct NormalState {
    pressed: heapless::FnvIndexMap<(u8, u8), (KeyCode, Option<KeyCode>), 8>,
}

impl NormalState {
    pub fn new() -> Self {
        Self {
            pressed: heapless::FnvIndexMap::new(),
        }
    }

    pub fn process_event(
        &mut self,
        event: &KeyChangeEvent,
        kc: (KeyCode, Option<KeyCode>),
        mut cb: impl FnMut(EventType, KeyCode),
    ) {
        if event.pressed {
            if self
                .pressed
                .insert((event.row, event.col), kc)
                .ok()
                .flatten()
                .is_none()
            {
                cb(EventType::Pressed, kc.0);
                if let Some(kc) = kc.1 {
                    cb(EventType::Pressed, kc);
                }
            }
        } else if self.pressed.remove(&(event.row, event.col)).is_some() {
            cb(EventType::Released, kc.0);
            if let Some(kc) = kc.1 {
                cb(EventType::Released, kc);
            }
        }
    }

    pub fn post_resolve(&mut self, mut cb: impl FnMut(EventType, KeyCode)) {
        for (_, kc) in self.pressed.iter() {
            cb(EventType::Pressing, kc.0);
            if let Some(kc) = kc.1 {
                cb(EventType::Pressing, kc);
            }
        }
    }
}
