use embassy_time::{Duration, Instant};

use crate::{
    config::static_config::CONFIG,
    keycode::{layer::LayerOp, KeyAction, KeyCode},
};

use super::{
    common::{CommonLocalState, CommonState},
    pressed::KeyStatusEvents,
};

struct KeyPressedState {
    pub press_start: Instant,
    pub action: KeyAction,
    pub force_hold: bool,
}

const DEFAULT_TAP_THRESHOLD: Duration = Duration::from_millis(CONFIG.default_tap_threshold);

/// Handles layer related events and resolve physical key position to keycode.
pub struct KeyResolver {
    key_state: [[Option<KeyPressedState>; CONFIG.cols * 2]; CONFIG.rows],
}

#[derive(Clone, Copy)]
pub enum EventType {
    Pressed,
    Pressing,
    Released,
}

impl KeyResolver {
    pub fn new() -> Self {
        Self {
            key_state: core::array::from_fn(|_| core::array::from_fn(|_| None)),
        }
    }

    pub fn resolve_key(
        &mut self,
        cs: &mut CommonState,
        cls: &CommonLocalState,
        events: &KeyStatusEvents,
    ) -> heapless::Vec<(EventType, KeyCode), 64> {
        use EventType::*;

        let highest_layer = cs.highest_layer();

        let mut resolved_keys = heapless::Vec::new();

        // If new key is pressed, all taphold keys in pressing state will be marked as force hold.
        // Same as QMK's HOLD_ON_OTHER_KEY_PRESS
        let force_hold = !events.pressed.is_empty();

        // pressing events are processed first to make sure that changed layer is used for prssed
        // events.
        for event in &events.pressing {
            if let Some(key_state) = &self.key_state[event.row as usize][event.col as usize] {
                match key_state.action {
                    KeyAction::Normal(kc) => {
                        let _ = resolved_keys.push((Pressing, kc));
                    }
                    KeyAction::Normal2(kc1, kc2) => {
                        let _ = resolved_keys.push((Pressing, kc1));
                        let _ = resolved_keys.push((Pressing, kc2));
                    }
                    KeyAction::TapHold(_tkc, hkc) => {
                        if key_state.force_hold
                            || cls.now - key_state.press_start > DEFAULT_TAP_THRESHOLD
                        {
                            let _ = resolved_keys.push((Pressing, hkc));
                        } else if force_hold {
                            self.key_state[event.row as usize][event.col as usize]
                                .as_mut()
                                .unwrap()
                                .force_hold = true;
                            let _ = resolved_keys.push((Pressing, hkc));
                        }
                    }
                }
            }
        }

        for event in &events.released {
            if let Some(key_state) = &self.key_state[event.row as usize][event.col as usize] {
                match key_state.action {
                    KeyAction::Normal(kc) => {
                        let _ = resolved_keys.push((Released, kc));
                    }
                    KeyAction::Normal2(kc1, kc2) => {
                        let _ = resolved_keys.push((Released, kc1));
                        let _ = resolved_keys.push((Released, kc2));
                    }
                    KeyAction::TapHold(tkc, hkc) => {
                        if key_state.force_hold
                            || cls.now - key_state.press_start > DEFAULT_TAP_THRESHOLD
                        {
                            let _ = resolved_keys.push((Released, hkc));
                        } else {
                            let _ = resolved_keys.push((Released, tkc));
                        }
                    }
                }
            }
            self.key_state[event.row as usize][event.col as usize] = None;
        }

        for (_ev, kc) in &resolved_keys {
            if let KeyCode::Layer(LayerOp::Move(l)) = kc {
                cs.layer_active[*l as usize] = true;
            }
        }

        let highest_layer = cs.highest_layer();
        for event in &events.pressed {
            let action = cs
                .get_keyaction(event.row, event.col, highest_layer)
                .unwrap();

            match action {
                KeyAction::Normal(kc) => {
                    let _ = resolved_keys.push((Pressed, kc));
                }
                KeyAction::Normal2(kc1, kc2) => {
                    let _ = resolved_keys.push((Pressed, kc1));
                    let _ = resolved_keys.push((Pressed, kc2));
                }
                KeyAction::TapHold(_, _) => {}
            };

            self.key_state[event.row as usize][event.col as usize] = Some(KeyPressedState {
                press_start: Instant::now(),
                action,
                force_hold: false,
            });
        }

        resolved_keys
    }
}
