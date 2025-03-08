//! Keyboard state management.

#![allow(clippy::let_unit_value)]
#![allow(clippy::single_match)]

use crate::{
    interface::{
        report::StateReport,
        state::{config::StateConfig, event::Event, KeymapInfo},
    },
    keymap::Keymap,
};
use key_resolver::EventType;
use manager::{GlobalManagerState, LocalManagerState};

mod key_resolver;
mod manager;
mod shared;

// TODO: Delete these generics hell in some day...

/// Represents the state of the keyboard.
pub struct State<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const ONESHOT_STATE_SIZE: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
> {
    key_resolver: key_resolver::KeyResolver<
        ONESHOT_STATE_SIZE,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >,
    shared: shared::SharedState<
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >,
    config: StateConfig,
    manager: GlobalManagerState,
}

impl<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
        const ONESHOT_STATE_SIZE: usize,
        const TAP_DANCE_MAX_DEFINITIONS: usize,
        const TAP_DANCE_MAX_REPEATS: usize,
        const COMBO_KEY_MAX_DEFINITIONS: usize,
        const COMBO_KEY_MAX_SOURCES: usize,
    >
    State<
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        ONESHOT_STATE_SIZE,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >
{
    /// Creates a new state with the given keymap and configuration.
    pub fn new(
        keymap: Keymap<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
        config: StateConfig,
    ) -> Self {
        const {
            assert!(LAYER >= 1, "Layer count must be at least 1");
        }

        Self {
            config: config.clone(),
            key_resolver: key_resolver::KeyResolver::new(
                config.key_resolver,
                keymap.tap_dance.clone(),
                keymap.combo.clone(),
            ),
            shared: shared::SharedState::new(keymap),
            manager: GlobalManagerState::new(config.mouse, config.initial_output),
        }
    }

    /// Updates state with the given events.
    pub fn update(&mut self, event: Event, since_last_update: core::time::Duration) -> StateReport {
        self.shared.now = self.shared.now + since_last_update.into();

        let mut lms = LocalManagerState::new(&self.manager);

        let key_change = match event {
            Event::Key(key_change) => Some(key_change),
            Event::Mouse(movement) => {
                lms.process_mouse_event(movement);
                None
            }
            Event::Encoder((id, dir)) => {
                if let Some(kc) =
                    self.shared
                        .keymap
                        .get_encoder_key(self.shared.layer_active, id as usize, dir)
                {
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
            &mut self.shared.layer_active,
            key_change.as_ref(),
            self.shared.now,
            |layer_active, et, kc| {
                lms.process_keycode(layer_active, &mut self.manager, &kc, et);
            },
        );

        lms.report(&mut self.shared, &mut self.manager)
    }

    pub fn get_keymap(
        &self,
    ) -> &Keymap<
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    > {
        &self.shared.keymap
    }

    pub fn get_config(&self) -> &StateConfig {
        &self.config
    }

    pub fn get_keymap_info() -> KeymapInfo {
        KeymapInfo {
            layer_count: LAYER as u8,
            max_tap_dance_key_count: TAP_DANCE_MAX_DEFINITIONS as u8,
            max_tap_dance_repeat_count: TAP_DANCE_MAX_REPEATS as u8,
            oneshot_state_size: ONESHOT_STATE_SIZE as u8,
        }
    }
}

#[cfg(test)]
mod tests;
