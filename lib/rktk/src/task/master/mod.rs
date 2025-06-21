use rktk_keymanager::state::hid_report::HidReportState;

use crate::{config::CONST_CONFIG, utils::Mutex};

pub(super) mod handle_keyboard;
pub(super) mod handle_mouse;
pub(super) mod handle_slave;
pub(super) mod report;
#[cfg(feature = "rrp")]
pub(super) mod rrp_server;
pub(super) mod utils;

type ConfiguredState = HidReportState<
    { CONST_CONFIG.key_manager.layer_count as usize },
    { CONST_CONFIG.keyboard.rows as usize },
    { CONST_CONFIG.keyboard.cols as usize },
    { CONST_CONFIG.keyboard.encoder_count as usize },
    { CONST_CONFIG.key_manager.normal_max_pressed_keys },
    { CONST_CONFIG.key_manager.oneshot_state_size },
    { CONST_CONFIG.key_manager.tap_dance_max_definitions },
    { CONST_CONFIG.key_manager.tap_dance_max_repeats },
    { CONST_CONFIG.key_manager.combo_key_max_definitions },
    { CONST_CONFIG.key_manager.combo_key_max_sources },
>;

type SharedState = Mutex<ConfiguredState>;
