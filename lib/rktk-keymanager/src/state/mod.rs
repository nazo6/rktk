//! Keyboard state management.

#![allow(clippy::let_unit_value)]
#![allow(clippy::single_match)]

use crate::{keymap::Keymap, time::Duration};
use config::{
    KeymapInfo, StateConfig, MAX_RESOLVED_KEY_COUNT, MAX_TAP_DANCE_KEY_COUNT,
    MAX_TAP_DANCE_REPEAT_COUNT, ONESHOT_STATE_SIZE,
};
use key_resolver::EventType;
use manager::{GlobalManagerState, LocalManagerState};

pub mod config;
mod interface;
mod key_resolver;
mod manager;
mod shared;

pub use interface::*;

fn size<T>(_: &T) {
    #[cfg(test)]
    dbg!(core::mem::size_of::<T>());
}

/// Represents the state of the keyboard.
pub struct State<const LAYER: usize, const ROW: usize, const COL: usize, const ENCODER_COUNT: usize>
{
    key_resolver: key_resolver::KeyResolver,
    shared: shared::SharedState<LAYER, ROW, COL, ENCODER_COUNT>,
    config: StateConfig,
    manager: GlobalManagerState,
}

impl<const LAYER: usize, const ROW: usize, const COL: usize, const ENCODER_COUNT: usize>
    State<LAYER, ROW, COL, ENCODER_COUNT>
{
    /// Creates a new state with the given keymap and configuration.
    pub fn new(keymap: Keymap<LAYER, ROW, COL, ENCODER_COUNT>, config: StateConfig) -> Self {
        let s = Self {
            config: config.clone(),
            key_resolver: key_resolver::KeyResolver::new(config.key_resolver),
            shared: shared::SharedState::new(keymap),
            manager: GlobalManagerState::new(config.mouse, config.initial_output),
        };
        size(&s.key_resolver);
        s
    }

    /// Updates state with the given events.
    pub fn update(&mut self, event: Event, since_last_update: Duration) -> StateReport {
        #[cfg(test)]
        dbg!("update");

        self.shared.now = self.shared.now + since_last_update;

        let mut lms = LocalManagerState::new();

        let key_change = match event {
            Event::Key(key_change) => Some(key_change),
            Event::Mouse(movement) => {
                lms.process_mouse_event(movement);
                None
            }
            Event::Encoder((id, dir)) => {
                if let Some(kc) = self.shared.keymap.get_encoder_key(id as usize, dir) {
                    lms.process_keycode(
                        &mut self.shared.layer_active,
                        &mut self.manager,
                        kc,
                        EventType::Pressed,
                    );
                    lms.process_keycode(
                        &mut self.shared.layer_active,
                        &mut self.manager,
                        kc,
                        EventType::Released,
                    );
                }
                None
            }
            _ => None,
        };

        self.key_resolver.resolve_key(
            &self.shared.keymap,
            self.shared.layer_active,
            key_change.as_ref(),
            self.shared.now,
            |et, kc| {
                #[cfg(test)]
                dbg!(&et, &kc);

                lms.process_keycode(&mut self.shared.layer_active, &mut self.manager, &kc, et);
            },
        );

        lms.report(&mut self.shared, &mut self.manager)
    }

    pub fn get_keymap(&self) -> &Keymap<LAYER, ROW, COL, ENCODER_COUNT> {
        &self.shared.keymap
    }

    pub fn get_config(&self) -> &StateConfig {
        &self.config
    }

    pub fn get_keymap_info() -> KeymapInfo {
        KeymapInfo {
            layer_count: LAYER as u8,
            max_tap_dance_key_count: MAX_TAP_DANCE_KEY_COUNT,
            max_tap_dance_repeat_count: MAX_TAP_DANCE_REPEAT_COUNT,
            oneshot_state_size: ONESHOT_STATE_SIZE,
            max_resolved_key_count: MAX_RESOLVED_KEY_COUNT,
        }
    }
}

#[cfg(test)]
mod tests;
