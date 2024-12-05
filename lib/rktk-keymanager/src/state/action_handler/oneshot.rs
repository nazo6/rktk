use crate::{
    keycode::KeyCode,
    state::{config::ONESHOT_STATE_SIZE, pressed::KeyLocation, KeyChangeEvent},
};

use super::EventType;

#[derive(Debug)]
struct OneshotKeyState {
    pub key: KeyCode,
    // When active, this is some and contains the location of the key that activated this oneshot
    // key.
    pub active: Option<KeyLocation>,
}

pub struct OneshotState {
    oneshot: crate::Vec<OneshotKeyState, { ONESHOT_STATE_SIZE as usize }>,
}

impl OneshotState {
    pub fn new() -> Self {
        Self {
            oneshot: crate::Vec::new(),
        }
    }

    pub fn pre_resolve(&mut self, event: &KeyChangeEvent, mut cb: impl FnMut(EventType, KeyCode)) {
        for oneshot in &mut self.oneshot {
            if event.pressed {
                if oneshot.active.is_none() {
                    oneshot.active = Some(KeyLocation {
                        row: event.row,
                        col: event.col,
                    });
                    cb(EventType::Pressed, oneshot.key);
                    continue;
                }
            } else if oneshot.active
                == Some(KeyLocation {
                    row: event.row,
                    col: event.col,
                })
            {
                oneshot.active = None;
                cb(EventType::Released, oneshot.key);
                continue;
            }

            if oneshot.active.is_some() {
                cb(EventType::Pressing, oneshot.key);
            }
        }
    }

    pub fn process_keycode(&mut self, kc: &KeyCode, pressed: bool) {
        if pressed {
            self.oneshot.push(OneshotKeyState {
                key: *kc,
                active: None,
            });
        }
    }
}
