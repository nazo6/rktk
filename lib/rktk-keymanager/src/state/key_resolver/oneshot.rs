use crate::{
    keycode::KeyCode,
    state::{config::ONESHOT_STATE_SIZE, KeyChangeEvent},
};

use super::EventType;

#[derive(Debug)]
struct OneshotKeyState {
    pub key: KeyCode,
    // When active, this is some and contains the location of the key that activated this oneshot
    // key.
    pub active: Option<(u8, u8)>,
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

    pub fn pre_resolve(
        &mut self,
        event: Option<&KeyChangeEvent>,
        mut cb: impl FnMut(EventType, KeyCode),
    ) {
        self.oneshot.retain_mut(|oneshot| {
            if let Some(event) = event {
                if event.pressed {
                    if oneshot.active.is_none() {
                        oneshot.active = Some((event.row, event.col));
                        cb(EventType::Pressed, oneshot.key);
                        return true;
                    }
                } else if oneshot.active == Some((event.row, event.col)) {
                    cb(EventType::Released, oneshot.key);
                    return false;
                }
            }

            if oneshot.active.is_some() {
                cb(EventType::Pressing, oneshot.key);
            }

            true
        });
    }

    pub fn process_keycode(&mut self, kc: &KeyCode, pressed: bool) {
        if pressed {
            let _ = self.oneshot.push(OneshotKeyState {
                key: *kc,
                active: None,
            });
        }
    }
}
