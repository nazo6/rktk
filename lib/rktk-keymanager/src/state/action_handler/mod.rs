use crate::keymap::Keymap;
use crate::time::{Duration, Instant};

use super::EncoderDirection;
use super::KeyChangeEvent;
use crate::keycode::{layer::LayerOp, KeyAction, KeyCode};

use super::{
    common::{CommonLocalState, CommonState},
    config::KeyResolverConfig,
};

mod normal;
mod oneshot;
mod tap_dance;
mod tap_hold;

#[derive(Debug)]
struct KeyPressedState {
    pub press_start: Instant,
    pub action: KeyAction,
    pub hold: bool,
}

/// Handles layer related events and resolve physical key position to keycode.
pub struct ActionHandler<const ROW: usize, const COL: usize> {
    key_state: [[Option<KeyPressedState>; COL]; ROW],
    normal_state: normal::NormalState,
    tap_dance: tap_dance::TapDanceState,
    oneshot: oneshot::OneshotState,
    tap_hold: tap_hold::TapHoldState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventType {
    Pressed,
    Pressing,
    Released,
}

impl<const ROW: usize, const COL: usize> ActionHandler<ROW, COL> {
    pub fn new(config: KeyResolverConfig) -> Self {
        Self {
            normal_state: normal::NormalState::new(),
            key_state: core::array::from_fn(|_| core::array::from_fn(|_| None)),
            tap_dance: tap_dance::TapDanceState::new(config.tap_dance),
            oneshot: oneshot::OneshotState::new(),
            tap_hold: tap_hold::TapHoldState::new(config.tap_threshold),
        }
    }

    pub fn resolve_key<const LAYER: usize, const ENCODER_COUNT: usize>(
        &mut self,
        keymap: &Keymap<LAYER, ROW, COL, ENCODER_COUNT>,
        layer_state: [bool; LAYER],
        event: &KeyChangeEvent,
        mut cb: impl FnMut(EventType, KeyCode),
        now: Instant,
    ) {
        self.oneshot.pre_resolve(event, &mut cb);

        let highest_layer = layer_state
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &active)| active)
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        let Some(mut key_action) =
            keymap.get_keyaction(highest_layer, event.row as usize, event.col as usize)
        else {
            return;
        };

        if *key_action == KeyAction::Inherit {
            for layer in 0..highest_layer {
                let Some(action) =
                    keymap.get_keyaction(layer, event.row as usize, event.col as usize)
                else {
                    return;
                };
                if *action != KeyAction::Inherit {
                    key_action = action;
                    break;
                }
            }
        }

        match key_action {
            KeyAction::Inherit => unreachable!(),
            KeyAction::Normal(key_code) => {
                self.normal_state
                    .process_event(event, (*key_code, None), &mut cb);
            }
            KeyAction::Normal2(key_code, key_code1) => {
                self.normal_state
                    .process_event(event, (*key_code, Some(*key_code1)), &mut cb);
            }
            KeyAction::TapHold(tkc, hkc) => {
                self.tap_hold
                    .process_event(now, event, (*tkc, *hkc), &mut cb);
            }
            KeyAction::OneShot(key_code) => {
                self.oneshot.process_keycode(key_code, event.pressed);
            }
            KeyAction::TapDance(id) => {
                self.tap_dance
                    .process_event(*id, now, event.pressed, &mut cb);
            }
        }
    }
}
