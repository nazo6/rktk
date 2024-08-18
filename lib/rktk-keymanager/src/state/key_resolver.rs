use embassy_time::{Duration, Instant};

use crate::keycode::{layer::LayerOp, KeyAction, KeyCode};

use super::{
    common::{CommonLocalState, CommonState},
    pressed::{KeyLocation, KeyStatusEvents},
};

struct KeyPressedState {
    pub press_start: Instant,
    pub action: KeyAction,
    pub hold: bool,
}

struct OneShotState {
    pub key: KeyCode,
    // When active, this is some and contains the location of the key that activated this oneshot
    // key.
    pub active: Option<KeyLocation>,
}

/// Handles layer related events and resolve physical key position to keycode.
pub struct KeyResolver<const ROW: usize, const COL: usize> {
    key_state: [[Option<KeyPressedState>; COL]; ROW],
    oneshot: heapless::Vec<OneShotState, 4>,
    tap_threshold: Duration,
}

#[derive(Clone, Copy)]
pub enum EventType {
    Pressed,
    Pressing,
    Released,
}

impl<const ROW: usize, const COL: usize> KeyResolver<ROW, COL> {
    pub fn new(tap_threshold: Duration) -> Self {
        Self {
            key_state: core::array::from_fn(|_| core::array::from_fn(|_| None)),
            oneshot: heapless::Vec::new(),
            tap_threshold,
        }
    }

    pub fn handle_layer_kc<const LAYER: usize>(
        common_state: &mut CommonState<LAYER, ROW, COL>,
        kc: &KeyCode,
        event: EventType,
    ) {
        match kc {
            KeyCode::Layer(layer_op) => match (event, layer_op) {
                (EventType::Pressed, LayerOp::Toggle(l)) => {
                    common_state.layer_active[*l as usize] =
                        !common_state.layer_active[*l as usize];
                }
                (EventType::Pressed, LayerOp::Momentary(l)) => {
                    common_state.layer_active[*l as usize] = true;
                }
                (EventType::Released, LayerOp::Momentary(l)) => {
                    common_state.layer_active[*l as usize] = false;
                }
                _ => {}
            },
            _ => {}
        };
    }

    pub fn resolve_key<const LAYER: usize>(
        &mut self,
        cs: &mut CommonState<LAYER, ROW, COL>,
        cls: &CommonLocalState,
        events: &KeyStatusEvents,
    ) -> heapless::Vec<(EventType, KeyCode), 64> {
        use EventType::*;

        let mut resolved_keys = heapless::Vec::new();

        if let Some(loc) = events.pressed.first() {
            for osc in &mut self.oneshot {
                if osc.active.is_none() {
                    osc.active = Some(*loc);
                    let _ = resolved_keys.push((Pressed, osc.key));
                }
            }
        }
        self.oneshot.retain(|osc| {
            if let Some(loc) = osc.active {
                if events.released.contains(&loc) {
                    let _ = resolved_keys.push((Released, osc.key));
                    return false;
                } else {
                    let _ = resolved_keys.push((Pressing, osc.key));
                }
            }
            true
        });

        // If new key is pressed, all taphold keys in pressing state will be marked as force hold.
        // Same as QMK's HOLD_ON_OTHER_KEY_PRESS
        let make_hold = !events.pressed.is_empty();

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
                        if key_state.hold {
                            let _ = resolved_keys.push((Pressing, hkc));
                        } else if cls.now - key_state.press_start > self.tap_threshold || make_hold
                        {
                            self.key_state[event.row as usize][event.col as usize]
                                .as_mut()
                                .unwrap()
                                .hold = true;
                            let _ = resolved_keys.push((Pressed, hkc));
                        }
                    }
                    KeyAction::OneShot(_) => {}
                    KeyAction::Inherit => unreachable!(),
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
                        if key_state.hold || cls.now - key_state.press_start > self.tap_threshold {
                            let _ = resolved_keys.push((Released, hkc));
                        } else {
                            let _ = resolved_keys.push((Pressed, tkc));
                        }
                    }
                    KeyAction::OneShot(_) => {}
                    KeyAction::Inherit => unreachable!(),
                }
            }
            self.key_state[event.row as usize][event.col as usize] = None;
        }

        // To determine layer for `pressed` keys, we have to apply the layer changed in above loop.
        // This is important to implement HOLD_ON_OTHER_KEY_PRESS and one shot layer.
        for (event, kc) in &resolved_keys {
            Self::handle_layer_kc(cs, kc, *event);
        }

        let highest_layer = cs.highest_layer();
        for event in &events.pressed {
            let Some(action) = cs.get_inherited_keyaction(event.row, event.col, highest_layer)
            else {
                continue;
            };

            match action {
                KeyAction::Normal(kc) => {
                    let _ = resolved_keys.push((Pressed, kc));
                }
                KeyAction::Normal2(kc1, kc2) => {
                    let _ = resolved_keys.push((Pressed, kc1));
                    let _ = resolved_keys.push((Pressed, kc2));
                }
                KeyAction::TapHold(_, _) => {}
                KeyAction::OneShot(kc) => {
                    let _ = self.oneshot.push(OneShotState {
                        key: kc,
                        active: None,
                    });
                }
                KeyAction::Inherit => unreachable!(),
            };

            self.key_state[event.row as usize][event.col as usize] = Some(KeyPressedState {
                press_start: Instant::now(),
                action,
                hold: false,
            });
        }

        for (event, kc) in &resolved_keys {
            Self::handle_layer_kc(cs, kc, *event);
        }

        resolved_keys
    }
}
