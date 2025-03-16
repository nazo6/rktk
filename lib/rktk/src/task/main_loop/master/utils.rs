use rktk_keymanager::interface::state::{config::StateConfig, input_event::KeyChangeEvent};
use rktk_log::helper::Debug2Format;

use crate::{
    config::{
        constant::{KEYBOARD, KM_CONFIG},
        keymap::Keymap,
        storage::StorageConfigManager,
    },
    drivers::interface::storage::StorageDriver,
    interface::Hand,
};

use super::{ConfiguredState, RKTK_CONFIG, SharedState};

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
                rktk_log::info!("Storage version matched");
                config_storage = Some(s);
            }
            Ok(i) => {
                rktk_log::warn!("Storage version matched");
                crate::print!("Storage version mismatch: {}", i);
            }
            Err(_e) => match s.write_version(1).await {
                Ok(_) => {
                    config_storage = Some(s);
                }
                Err(e) => {
                    rktk_log::error!(
                        "Storage to write version to storage: {:?}",
                        Debug2Format(&e)
                    );
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
    mut keymap: Keymap,
) -> SharedState {
    let (state_config, keymap) = if let Some(storage) = &config_store {
        for l in 0..RKTK_CONFIG.layer_count {
            if let Ok(layer) = storage.read_keymap(l).await {
                keymap.layers[l as usize] = layer;
            }
        }

        let c = storage.read_state_config().await;

        (c.ok(), keymap)
    } else {
        (None, keymap)
    };

    let state_config = state_config.unwrap_or(StateConfig {
        mouse: KM_CONFIG.mouse,
        key_resolver: KM_CONFIG.key_resolver,
    });

    SharedState::new(ConfiguredState::new(keymap, state_config))
}
