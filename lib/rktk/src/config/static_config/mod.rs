//! Static configuration of the firmware.
//!
//! These values are read from a json file set in the environment variable `RKTK_CONFIG_PATH`
//! and set at build time.
//!
//! It is convenient to set environment variables in `.cargo/config.toml` as follows.
//! ```toml
//! [env]
//! RKTK_CONFIG_PATH = { value = "rktk.json", relative = true }
//! ```
//! See the examples folder for an example of this json.

mod schema;

use rktk_keymanager::state::config::{
    KeyResolverConfig, MouseConfig, TapDanceConfig, TapHoldConfig,
};

use embassy_time::Duration;

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

pub const KEYBOARD: schema::Keyboard = CONFIG.keyboard;
pub(crate) const RKTK_CONFIG: schema::RktkConfig = CONFIG.config.rktk;
pub(crate) const KM_CONFIG: schema::KeyManagerConfig = CONFIG.config.key_manager;

pub const SCAN_INTERVAL_KEYBOARD: Duration =
    Duration::from_millis(RKTK_CONFIG.scan_interval_keyboard);
pub const SCAN_INTERVAL_MOUSE: Duration = Duration::from_millis(RKTK_CONFIG.scan_interval_mouse);
