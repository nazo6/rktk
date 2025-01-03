use rktk_keymanager::state::{
    config::{Output, StateConfig},
    KeyChangeEvent,
};

use crate::{
    config::{Config, CONST_CONFIG},
    drivers::interface::{keyscan::Hand, storage::StorageDriver},
    keymap_config::Keymap,
    task::module::storage_config::StorageConfigManager,
};

use super::{ConfiguredState, SharedState};

const SPLIT_RIGHT_SHIFT: u8 = {
    if let Some(val) = CONST_CONFIG.split_right_shift {
        val
    } else {
        assert!(
            CONST_CONFIG.cols % 2 == 0,
            "Split right shift is not defined, but the keyboard has odd number of columns."
        );
        CONST_CONFIG.cols / 2
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
    mut keymap: Keymap,
    initial_output: Output,
    config: &'static Config,
) -> SharedState {
    let (state_config, keymap) = if let Some(storage) = &config_store {
        for l in 0..CONST_CONFIG.layer_count {
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
        mouse: config.key_manager.mouse.clone(),
        key_resolver: config.key_manager.key_resolver.clone(),
        initial_output,
    });

    SharedState::new(ConfiguredState::new(keymap, state_config))
}
