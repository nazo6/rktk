//! Keyboard state management.

#![allow(clippy::let_unit_value)]
#![allow(clippy::single_match)]

use crate::{
    interface::state::{
        KeymapInfo,
        config::StateConfig,
        input_event::InputEvent,
        output_event::{EventType, OutputEvent},
    },
    keymap::Keymap,
};

pub mod hid_report;
mod key_resolver;
mod shared;
mod updater;

// TODO: Delete these generics hell in some day...

/// Represents the state of the keyboard.
pub struct State<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const NORMAL_MAX_PRESSED_KEYS: usize,
    const ONESHOT_STATE_SIZE: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
> {
    key_resolver: key_resolver::KeyResolver<
        NORMAL_MAX_PRESSED_KEYS,
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
    updater_state: updater::UpdaterState,
}

impl<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const NORMAL_MAX_PRESSED_KEYS: usize,
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
        NORMAL_MAX_PRESSED_KEYS,
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
            updater_state: updater::UpdaterState::new(config.mouse),
        }
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

    pub fn get_layer_active(&self) -> &shared::LayerActive<LAYER> {
        &self.shared.layer_active
    }

    pub fn get_keymap_info() -> KeymapInfo {
        KeymapInfo {
            layer_count: LAYER as u8,
            max_tap_dance_key_count: TAP_DANCE_MAX_DEFINITIONS as u8,
            max_tap_dance_repeat_count: TAP_DANCE_MAX_REPEATS as u8,
            oneshot_state_size: ONESHOT_STATE_SIZE as u8,
        }
    }

    pub fn update(
        &mut self,
        event: InputEvent,
        since_last_update: core::time::Duration,
        mut cb: impl FnMut(OutputEvent),
    ) {
        self.shared.now = self.shared.now + since_last_update.into();
        let mut updater = self.updater_state.start_update();

        let key_change = match event {
            InputEvent::Key(key_change) => Some(key_change),
            InputEvent::Mouse(movement) => {
                updater.update_by_mouse_move(movement, &mut cb);
                None
            }
            InputEvent::Encoder((id, dir)) => {
                if let Some(kc) = self
                    .shared
                    .keymap
                    .get_encoder_key(self.shared.layer_active, id as usize, dir)
                    .copied()
                {
                    updater.update_by_keycode(&kc, EventType::Pressed, &mut self.shared, &mut cb);
                    updater.update_by_keycode(&kc, EventType::Released, &mut self.shared, &mut cb);
                }
                None
            }
            _ => None,
        };

        self.key_resolver
            .resolve_key(&mut self.shared, key_change.as_ref(), |shared, et, kc| {
                updater.update_by_keycode(&kc, et, shared, &mut cb);
            });

        updater.end(self.shared.highest_layer(), &mut self.shared, cb);
    }
}

#[cfg(test)]
mod tests;
