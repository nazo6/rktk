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

use embassy_time::Duration;

include!(concat!(env!("OUT_DIR"), "/config.rs"));

pub(crate) const KEYBOARD: schema::Keyboard = CONFIG.keyboard;
pub(crate) const RKTK_CONFIG: schema::RktkConfig = CONFIG.rktk;
pub(crate) const KM_CONFIG: schema::KeyManagerConfig = CONFIG.key_manager;

pub(crate) const SCAN_INTERVAL_KEYBOARD: Duration =
    Duration::from_millis(RKTK_CONFIG.scan_interval_keyboard);
pub(crate) const SCAN_INTERVAL_MOUSE: Duration =
    Duration::from_millis(RKTK_CONFIG.scan_interval_mouse);
