use heapless::index_map::FnvIndexMap;

use crate::{
    interface::state::{config::TapHoldConfig, input_event::KeyChangeEvent, output_event::EventType},
    keycode::KeyCode,
    time::{Duration, Instant},
};

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
    pressed: FnvIndexMap<(u8, u8), TapHoldKeyState, 16>,
    threshold: Duration,
    hold_on_other_key: bool,
}

impl TapHoldState {
    pub fn new(config: TapHoldConfig) -> Self {
        Self {
            pressed: FnvIndexMap::new(),
            threshold: Duration::from_millis(config.threshold),
            hold_on_other_key: config.hold_on_other_key,
        }
    }

    pub fn pre_resolve(
        &mut self,
        event: Option<&KeyChangeEvent>,
        now: Instant,
        out: &mut heapless::Vec<(KeyCode, EventType), 16>,
    ) {
        if let Some(event) = event
            && event.pressed
            && self.hold_on_other_key
        {
            for (key, state) in self.pressed.iter_mut() {
                if *key != (event.row, event.col) {
                    state.pending = None;
                }
            }
        }

        for (_, state) in self.pressed.iter_mut() {
            if let Some(press_start) = state.pending {
                if now - press_start > self.threshold {
                    // threshold reached, it's a hold.
                    let _ = out.push((state.hkc, EventType::Pressed));
                    state.pending = None;
                }
            } else {
                let _ = out.push((state.hkc, EventType::Pressing));
            }
        }
    }

    pub fn process_event(
        &mut self,
        now: Instant,
        event: &KeyChangeEvent,
        (tkc, hkc): (KeyCode, KeyCode),
        out: &mut heapless::Vec<(KeyCode, EventType), 16>,
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
                let _ = out.push((state.tkc, EventType::Pressed));
                let _ = out.push((state.tkc, EventType::Released));
            } else {
                // released in holding term. it's a hold.
                let _ = out.push((state.hkc, EventType::Released));
            }
        }
    }
}

