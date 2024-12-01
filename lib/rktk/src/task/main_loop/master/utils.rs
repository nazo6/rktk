use rktk_keymanager::state::{
    config::{KeyResolverConfig, MouseConfig, Output, StateConfig},
    KeyChangeEvent, StateReport,
};

use crate::{
    config::{static_config::KEYBOARD, storage_config::StorageConfigManager},
    drivers::interface::{
        backlight::{BacklightCommand, BacklightMode},
        keyscan::Hand,
        split::MasterToSlave,
        storage::StorageDriver,
    },
    hooks::M2sTx,
    task::backlight::BACKLIGHT_CTRL,
    KeyConfig,
};

use super::{ConfiguredState, SharedState, ThreadModeMutex, RKTK_CONFIG};

/// TODO: Currently, split index is changed like below.
/// Splitted:
/// 0 1 2 3 4   4 3 2 1 0
/// ↓
/// Entire:
/// 0 1 2 3 4   5 6 7 8 9
///
/// I'm not sure this is a common practice.
pub fn resolve_entire_key_pos(ev: &mut KeyChangeEvent, hand: Hand) {
    if hand == Hand::Right {
        ev.col = KEYBOARD.cols - 1 - ev.col;
    }
}

pub fn handle_led(
    state_report: &StateReport,
    m2s_tx: M2sTx<'_>,
    latest_led: &mut Option<BacklightCommand>,
) {
    let led = match state_report.highest_layer {
        1 => BacklightCommand::Start(BacklightMode::SolidColor(0, 0, 1)),
        2 => BacklightCommand::Start(BacklightMode::SolidColor(1, 0, 0)),
        3 => BacklightCommand::Start(BacklightMode::SolidColor(0, 1, 0)),
        4 => BacklightCommand::Start(BacklightMode::SolidColor(1, 1, 0)),
        _ => BacklightCommand::Reset,
    };

    if let Some(latest_led) = &latest_led {
        if led != *latest_led {
            let _ = BACKLIGHT_CTRL.try_send(led.clone());
            let _ = m2s_tx.try_send(MasterToSlave::Backlight(led.clone()));
        }
    }

    *latest_led = Some(led);
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

    let state_config = state_config.unwrap_or_else(|| StateConfig {
        mouse: MouseConfig {
            auto_mouse_layer: RKTK_CONFIG.default_auto_mouse_layer,
            auto_mouse_duration: RKTK_CONFIG.default_auto_mouse_duration,
            auto_mouse_threshold: RKTK_CONFIG.default_auto_mouse_threshold,
            scroll_divider_x: RKTK_CONFIG.default_scroll_divider_x,
            scroll_divider_y: RKTK_CONFIG.default_scroll_divider_y,
        },
        key_resolver: KeyResolverConfig {
            tap_threshold: RKTK_CONFIG.default_tap_threshold,
            tap_dance_threshold: RKTK_CONFIG.default_tap_dance_threshold,
            tap_dance: key_config.tap_dance.clone(),
        },
        initial_output,
    });

    ThreadModeMutex::new(ConfiguredState::new(keymap, state_config))
}
