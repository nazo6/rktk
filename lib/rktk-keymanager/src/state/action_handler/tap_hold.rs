use core::time::Duration;

use crate::{keycode::KeyCode, state::KeyChangeEvent, time::Instant};

use super::EventType;

struct TapHoldKeyState {
    tkc: KeyCode,
    hkc: KeyCode,
    // If Some, key is waiting for threshold to be reached
    // If none, key is in holding state
    pending: Option<Instant>,
}

/// State management for TapHold and Normal2 action
pub struct TapHoldState {
    pressed: heapless::FnvIndexMap<(u8, u8), TapHoldKeyState, 10>,
    threshold: Duration,
}

impl TapHoldState {
    pub fn new(threshold: u32) -> Self {
        Self {
            pressed: heapless::FnvIndexMap::new(),
            threshold: Duration::from_millis(threshold as u64),
        }
    }

    pub fn process_event(
        &mut self,
        now: Instant,
        event: &KeyChangeEvent,
        (tkc, hkc): (KeyCode, KeyCode),
        mut cb: impl FnMut(EventType, KeyCode),
    ) {
        if event.pressed {
            self.pressed.insert(
                (event.row, event.col),
                TapHoldKeyState {
                    tkc,
                    hkc,
                    pending: Some(now),
                },
            );
        } else if let Some(state) = self.pressed.remove(&(event.row, event.col)) {
            if state.pending.is_some() {
                // released in tapping term. it's a tap.
                cb(EventType::Pressed, state.tkc);
                cb(EventType::Released, state.tkc);
            } else {
                // released in holding term. it's a hold.
                cb(EventType::Released, state.hkc);
            }
        }
    }

    pub fn post_resolve(&mut self, now: Instant, mut cb: impl FnMut(EventType, KeyCode)) {
        for (_, state) in self.pressed.iter_mut() {
            if let Some(pending_dur) = state.pending {
                if now - pending_dur > self.threshold {
                    // threshold reached, it's a hold.
                    cb(EventType::Pressed, state.hkc);
                    state.pending = None;
                }
            } else {
                cb(EventType::Pressing, state.hkc);
            }
        }
    }
}
