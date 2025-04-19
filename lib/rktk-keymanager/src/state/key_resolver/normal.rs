use crate::{interface::state::input_event::KeyChangeEvent, keycode::KeyCode};

use super::EventType;

#[derive(PartialEq, Eq)]
struct NormalKeyState {
    col: u8,
    row: u8,
    k1: KeyCode,
    k2: Option<KeyCode>,
}

/// State management for Normal and Normal2 action
pub struct NormalState {
    pressed: heapless::Vec<NormalKeyState, 8>,
}

impl NormalState {
    pub fn new() -> Self {
        Self {
            pressed: heapless::Vec::new(),
        }
    }

    pub fn process_event(
        &mut self,
        event: &KeyChangeEvent,
        kc: (KeyCode, Option<KeyCode>),
        mut cb: impl FnMut(EventType, KeyCode),
    ) {
        let new_key_state = NormalKeyState {
            col: event.col,
            row: event.row,
            k1: kc.0,
            k2: kc.1,
        };

        if event.pressed && !self.pressed.contains(&new_key_state) {
            self.pressed.push(new_key_state).ok();

            cb(EventType::Pressed, kc.0);
            if let Some(kc) = kc.1 {
                cb(EventType::Pressed, kc);
            }
        } else if !event.pressed {
            // release all keys that are pressed even if in other layers
            self.pressed.retain(|k| {
                if event.col == k.col && event.row == k.row {
                    cb(EventType::Released, k.k1);
                    if let Some(kc) = k.k2 {
                        cb(EventType::Released, kc);
                    }

                    false
                } else {
                    true
                }
            });
        }
    }

    pub fn post_resolve(&mut self, mut cb: impl FnMut(EventType, KeyCode)) {
        for s in self.pressed.iter() {
            cb(EventType::Pressing, s.k1);
            if let Some(k2) = s.k2 {
                cb(EventType::Pressing, k2);
            }
        }
    }
}
