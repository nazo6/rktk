use crate::keymap::{Keymap, TapDanceDefinitions};
use crate::time::Instant;

use super::shared::LayerActive;
use super::KeyChangeEvent;
use crate::keycode::{KeyAction, KeyCode};

use super::config::KeyResolverConfig;

mod normal;
mod oneshot;
mod tap_dance;
mod tap_hold;

/// Handles layer related events and resolve physical key position to keycode.
pub struct KeyResolver {
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

impl KeyResolver {
    pub fn new(config: KeyResolverConfig, tap_dance_def: TapDanceDefinitions) -> Self {
        Self {
            normal_state: normal::NormalState::new(),
            tap_dance: tap_dance::TapDanceState::new(tap_dance_def, config.tap_dance),
            oneshot: oneshot::OneshotState::new(),
            tap_hold: tap_hold::TapHoldState::new(config.tap_hold),
        }
    }

    pub fn resolve_key<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
    >(
        &mut self,
        keymap: &Keymap<LAYER, ROW, COL, ENCODER_COUNT>,
        layer_state: &mut LayerActive<LAYER>,
        event: Option<&KeyChangeEvent>,
        now: Instant,
        mut cb: impl FnMut(&mut LayerActive<LAYER>, EventType, KeyCode),
    ) {
        self.oneshot
            .pre_resolve(event, &mut |event_type, key_code| {
                cb(layer_state, event_type, key_code);
            });
        self.tap_hold
            .pre_resolve(event, now, &mut |event_type, key_code| {
                cb(layer_state, event_type, key_code);
            });

        let highest_layer = layer_state
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &active)| active)
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        let mut cb_with_layer = |event_type, key_code| {
            cb(layer_state, event_type, key_code);
        };

        if let Some(event) = event {
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
                KeyAction::Inherit => {}
                KeyAction::Normal(key_code) => {
                    self.normal_state
                        .process_event(event, (*key_code, None), &mut cb_with_layer);
                }
                KeyAction::Normal2(key_code, key_code1) => {
                    self.normal_state.process_event(
                        event,
                        (*key_code, Some(*key_code1)),
                        &mut cb_with_layer,
                    );
                }
                KeyAction::TapHold(tkc, hkc) => {
                    self.tap_hold
                        .process_event(now, event, (*tkc, *hkc), &mut cb_with_layer);
                }
                KeyAction::OneShot(key_code) => {
                    self.oneshot.process_keycode(key_code, event.pressed);
                }
                KeyAction::TapDance(id) => {
                    self.tap_dance
                        .process_event(*id, now, event.pressed, &mut cb_with_layer);
                }
            }
        }

        self.tap_dance.post_resolve(now, &mut cb_with_layer);
        self.normal_state.post_resolve(&mut cb_with_layer);
    }
}
