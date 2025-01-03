use crate::{
    config::TapHoldConfig,
    keycode::KeyCode,
    state::KeyChangeEvent,
    time::{Duration, Instant},
};

use super::EventType;

#[derive(Debug)]
struct TapHoldKeyState {
    tkc: KeyCode,
    hkc: KeyCode,
    // If Some, key is waiting for threshold to be reached
    // If none, key is in holding state
    pending: Option<Instant>,
}

/// State management for TapHold and Normal2 action
pub struct TapHoldState {
    pressed: heapless::FnvIndexMap<(u8, u8), TapHoldKeyState, 16>,
    threshold: Duration,
    hold_on_other_key: bool,
}

impl TapHoldState {
    pub fn new(config: TapHoldConfig) -> Self {
        Self {
            pressed: heapless::FnvIndexMap::new(),
            threshold: Duration::from_millis(config.threshold),
            hold_on_other_key: config.hold_on_other_key,
        }
    }

    pub fn pre_resolve(
        &mut self,
        event: Option<&KeyChangeEvent>,

        now: Instant,
        mut cb: impl FnMut(EventType, KeyCode),
    ) {
        if let Some(event) = event {
            if event.pressed && self.hold_on_other_key {
                for (key, state) in self.pressed.iter_mut() {
                    if *key != (event.row, event.col) {
                        state.pending = None;
                    }
                }
            }
        }

        for (_, state) in self.pressed.iter_mut() {
            if let Some(press_start) = state.pending {
                if now - press_start > self.threshold {
                    // threshold reached, it's a hold.
                    cb(EventType::Pressed, state.hkc);
                    state.pending = None;
                }
            } else {
                cb(EventType::Pressing, state.hkc);
            }
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
            let _ = self.pressed.insert(
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
}
