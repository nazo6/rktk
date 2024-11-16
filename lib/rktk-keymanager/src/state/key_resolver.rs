use crate::time::{Duration, Instant};

use super::config::{
    TapDanceConfig, MAX_RESOLVED_KEY_COUNT, MAX_TAP_DANCE_REPEAT_COUNT, ONESHOT_STATE_SIZE,
};
use crate::keycode::{layer::LayerOp, KeyAction, KeyCode};

use super::{
    common::{CommonLocalState, CommonState},
    config::KeyResolverConfig,
    pressed::{KeyLocation, KeyStatusEvents},
};

#[derive(Debug)]
struct KeyPressedState {
    pub press_start: Instant,
    pub action: KeyAction,
    pub hold: bool,
}

#[derive(Debug)]
struct OneShotState {
    pub key: KeyCode,
    // When active, this is some and contains the location of the key that activated this oneshot
    // key.
    pub active: Option<KeyLocation>,
}

#[derive(Debug)]
struct TapDance {
    pub state: Option<TapDanceActiveState>,
    pub config: Option<TapDanceConfig>,
}

#[derive(Debug)]
struct TapDanceActiveState {
    pub waiting: bool,
    pub tap_count: u8,
    pub last_release: Instant,
}

/// Handles layer related events and resolve physical key position to keycode.
pub struct KeyResolver<const ROW: usize, const COL: usize> {
    key_state: [[Option<KeyPressedState>; COL]; ROW],
    tap_dance: [TapDance; MAX_TAP_DANCE_REPEAT_COUNT as usize],
    oneshot: heapless::Vec<OneShotState, { ONESHOT_STATE_SIZE as usize }>,
    tap_threshold: Duration,
    tap_dance_threshold: Duration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventType {
    Pressed,
    Pressing,
    Released,
}

impl<const ROW: usize, const COL: usize> KeyResolver<ROW, COL> {
    pub fn new(mut config: KeyResolverConfig) -> Self {
        Self {
            key_state: core::array::from_fn(|_| core::array::from_fn(|_| None)),
            oneshot: heapless::Vec::new(),
            tap_dance: core::array::from_fn(|i| TapDance {
                state: None,
                config: config.tap_dance[i].take(),
            }),
            tap_threshold: Duration::from_millis(config.tap_threshold as u64),
            tap_dance_threshold: Duration::from_millis(config.tap_dance_threshold as u64),
        }
    }

    /// Give keycode and handle it if it is layer related key.
    /// returns true if it is layer related key.
    pub fn handle_layer_kc<const LAYER: usize, const ENCODER_COUNT: usize>(
        common_state: &mut CommonState<LAYER, ROW, COL, ENCODER_COUNT>,
        kc: &KeyCode,
        event: EventType,
    ) -> bool {
        match kc {
            KeyCode::Layer(layer_op) => {
                match (event, layer_op) {
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
                }
                true
            }
            _ => false,
        }
    }

    pub fn resolve_key<const LAYER: usize, const ENCODER_COUNT: usize>(
        &mut self,
        cs: &mut CommonState<LAYER, ROW, COL, ENCODER_COUNT>,
        cls: &CommonLocalState,
        key_events: &KeyStatusEvents,
        encoder_events: &[(usize, super::EncoderDirection)],
    ) -> heapless::Vec<(EventType, KeyCode), { MAX_RESOLVED_KEY_COUNT as usize }> {
        use EventType::*;

        let mut resolved_keys = heapless::Vec::new();

        if let Some(loc) = key_events.pressed.first() {
            for osc in &mut self.oneshot {
                if osc.active.is_none() {
                    osc.active = Some(*loc);
                    let _ = resolved_keys.push((Pressed, osc.key));
                }
            }
        }
        self.oneshot.retain(|osc| {
            if let Some(loc) = osc.active {
                if key_events.released.contains(&loc) {
                    let _ = resolved_keys.push((Released, osc.key));
                    return false;
                } else {
                    let _ = resolved_keys.push((Pressing, osc.key));
                }
            }
            true
        });

        for td in self.tap_dance.iter_mut() {
            if let Some(tds) = &mut td.state {
                if tds.waiting && cls.now - tds.last_release > self.tap_dance_threshold {
                    if let Some(Some(Some(tkc))) = td
                        .config
                        .as_ref()
                        .map(|tdc| tdc.tap.get(tds.tap_count as usize - 1))
                    {
                        let _ = resolved_keys.push((Pressed, *tkc));
                        let _ = resolved_keys.push((Released, *tkc));
                    }

                    td.state = None;
                }
            }
        }

        // If new key is pressed, all taphold keys in pressing state will be marked as force hold.
        // Same as QMK's HOLD_ON_OTHER_KEY_PRESS
        let make_hold = !key_events.pressed.is_empty();

        for event in &key_events.pressing {
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
                    KeyAction::TapDance(id) => {
                        if let Some(td) = self.tap_dance.get_mut(id as usize) {
                            if let (Some(config), Some(tap_count)) =
                                (&mut td.config, td.state.as_ref().map(|tds| tds.tap_count))
                            {
                                if let Some(Some(hkc)) = config.hold.get(tap_count as usize - 1) {
                                    if key_state.hold {
                                        let _ = resolved_keys.push((Pressing, *hkc));
                                    } else if cls.now - key_state.press_start > self.tap_threshold
                                        || make_hold
                                    {
                                        self.key_state[event.row as usize][event.col as usize]
                                            .as_mut()
                                            .unwrap()
                                            .hold = true;
                                        let _ = resolved_keys.push((Pressed, *hkc));
                                    }
                                }
                            }
                        }
                    }
                    KeyAction::Inherit => unreachable!(),
                }
            }
        }

        for event in &key_events.released {
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
                            let _ = resolved_keys.push((Released, tkc));
                        }
                    }
                    KeyAction::OneShot(_) => {}
                    KeyAction::TapDance(id) => {
                        if let Some(td) = self.tap_dance.get_mut(id as usize) {
                            if let (Some(config), Some(tds)) = (&td.config, td.state.as_mut()) {
                                if key_state.hold
                                    || cls.now - key_state.press_start > self.tap_threshold
                                {
                                    if let Some(Some(hkc)) =
                                        config.hold.get(tds.tap_count as usize - 1)
                                    {
                                        let _ = resolved_keys.push((Released, *hkc));
                                        td.state = None;
                                    }
                                } else {
                                    tds.last_release = cls.now;
                                    tds.waiting = true;
                                }
                            }
                        }
                    }
                    KeyAction::Inherit => unreachable!(),
                }
            }
            self.key_state[event.row as usize][event.col as usize] = None;
        }

        for (encoder_id, encoder_dir) in encoder_events {
            let key = match encoder_dir {
                super::EncoderDirection::Clockwise => cs.keymap.encoder_keys[*encoder_id].1,
                super::EncoderDirection::CounterClockwise => cs.keymap.encoder_keys[*encoder_id].0,
            };

            let _ = resolved_keys.push((Pressed, key));
            let _ = resolved_keys.push((Released, key));
        }

        // To determine layer for `pressed` keys, we have to apply the layer changed in above loop.
        // This is important to implement HOLD_ON_OTHER_KEY_PRESS and one shot layer.
        resolved_keys.retain(|(ev, kc)| !Self::handle_layer_kc(cs, kc, *ev));

        let highest_layer = cs.highest_layer();
        for event in &key_events.pressed {
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
                KeyAction::TapDance(id) => {
                    if let Some(tap_dance_state) =
                        self.tap_dance.get_mut(id as usize).map(|td| &mut td.state)
                    {
                        match tap_dance_state {
                            Some(tds) => {
                                if cls.now - tds.last_release > self.tap_dance_threshold {
                                    tds.tap_count = 1;
                                } else {
                                    tds.tap_count += 1;
                                }
                                tds.waiting = false;
                            }
                            None => {
                                *tap_dance_state = Some(TapDanceActiveState {
                                    tap_count: 1,
                                    last_release: cls.now,
                                    waiting: false,
                                });
                            }
                        }
                    }
                }
                KeyAction::Inherit => unreachable!(),
            };

            self.key_state[event.row as usize][event.col as usize] = Some(KeyPressedState {
                press_start: cls.now,
                action,
                hold: false,
            });
        }

        resolved_keys.retain(|(ev, kc)| !Self::handle_layer_kc(cs, kc, *ev));

        resolved_keys
    }
}
