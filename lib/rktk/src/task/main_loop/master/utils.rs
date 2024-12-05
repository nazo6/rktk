use rktk_keymanager::state::{
    config::{KeyResolverConfig, MouseConfig, Output, StateConfig, TapDanceConfig},
    KeyChangeEvent,
};

use crate::{
    config::{static_config::KEYBOARD, storage_config::StorageConfigManager},
    drivers::interface::{keyscan::Hand, storage::StorageDriver},
    keymap_config::KeyConfig,
};

use super::{ConfiguredState, SharedState, RKTK_CONFIG};

const SPLIT_RIGHT_SHIFT: u8 = {
    if let Some(val) = KEYBOARD.split_right_shift {
        val
    } else {
        assert!(
            KEYBOARD.cols % 2 == 0,
            "Split right shift is not defined, but the keyboard has odd number of columns."
        );
        KEYBOARD.cols / 2
    }
};

/// Resolves one-handed coordinates to two-handed coordinates using split_right_shift.
pub fn resolve_entire_key_pos(ev: &mut KeyChangeEvent, hand: Hand) {
    if hand == Hand::Right {
        ev.col += SPLIT_RIGHT_SHIFT;
    }
}

/// Initialise storage as configuration manager
pub async fn init_storage<S: StorageDriver>(storage: Option<S>) -> Option<StorageConfigManager<S>> {
    let mut config_storage = None;
    if let Some(s) = storage {
        let s = StorageConfigManager::new(s);

        match s.read_version().await {
            Ok(1) => {
                log::info!("Storage version matched");
                config_storage = Some(s);
            }
            Ok(i) => {
                log::warn!("Storage version matched");
                crate::print!("Storage version mismatch: {}", i);
            }
            Err(_e) => match s.write_version(1).await {
                Ok(_) => {
                    config_storage = Some(s);
                }
                Err(e) => {
                    log::error!("Storage to write version to storage: {:?}", e);
                    crate::print!("Failed to access storage: {:?}", e);
                }
            },
        }
    }

    config_storage
}

/// Loads config from storage and return it as state.
/// If storage doesn't exist or read fails, uses provided static config value insted.
pub async fn load_state(
    config_store: &Option<StorageConfigManager<impl StorageDriver>>,
    key_config: KeyConfig,
    initial_output: Output,
) -> SharedState {
    let (state_config, keymap) = if let Some(storage) = &config_store {
        let mut keymap = key_config.keymap;
        for l in 0..RKTK_CONFIG.layer_count {
            if let Ok(layer) = storage.read_keymap(l).await {
                keymap.layers[l as usize] = layer;
            }
        }

        let c = storage.read_state_config().await;

        (c.ok(), keymap)
    } else {
        (None, key_config.keymap)
    };

    let state_config = state_config.unwrap_or(StateConfig {
        mouse: MouseConfig {
            auto_mouse_layer: RKTK_CONFIG.default_auto_mouse_layer,
            auto_mouse_duration: RKTK_CONFIG.default_auto_mouse_duration,
            auto_mouse_threshold: RKTK_CONFIG.default_auto_mouse_threshold,
            scroll_divider_x: RKTK_CONFIG.default_scroll_divider_x,
            scroll_divider_y: RKTK_CONFIG.default_scroll_divider_y,
        },
        key_resolver: KeyResolverConfig {
            tap_threshold: RKTK_CONFIG.default_tap_threshold,
            tap_dance: TapDanceConfig {
                definitions: key_config.tap_dance,
                threshold: RKTK_CONFIG.default_tap_dance_threshold,
            },
        },
        initial_output,
    });

    SharedState::new(ConfiguredState::new(keymap, state_config))
}
