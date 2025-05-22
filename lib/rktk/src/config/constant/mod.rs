//! Constant(static) configuration of the firmware.
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
//!
//! For each configuration, see the [`schema`] module.

include!(concat!(env!("OUT_DIR"), "/config.rs"));

pub(crate) const CONST_CONFIG: schema::ConstantConfig = CONFIG.constant;
