//! Rktk configuration management.
//!
//! # Overview of config
//!
//! The overall picture of rktk's configuration is shown below. Some can be configured by JSON files only, some by Rust code only, and some by both.
//!
//! ```text
//! rktk config
//! ├─ RktkOpts     <── Root option structure that is passed to the start function
//! │  ├─ config             <── This is "dynamic config". Can be set by both json and code
//! │  ├─ keymap             ┐
//! │  ├─ display            ├── Must be defined by the code
//! │  ├─ ... (Other opts)   ┘
//! ├─ CONST_CONFIG          <── This is "constant config". Automatically loaded from json file
//! ```
//!
//! # JSON file
//!
//! rktk uses a JSON file as the main config file. This JSON file is read at compile time from the path set in the `RKTK_CONFIG_PATH` environment variable.
//! This file allows both const config and dynamic config, with the difference that const config can only be configured in this file, while dynamic config is not.
//! Also, const config is used automatically, whereas dynamic config is not (see the `Dynamic config` section for details)
//!
//! You can configure your keyboard crate (bin crate) to use the `rktk.json` file in the project root by setting the following in `.cargo/config.toml`.
//! The file name can be anything, but we recommend `rktk.json` for clarity.
//!
//! ```toml
//! [env]
//! RKTK_CONFIG_PATH = { value = "rktk.json", relative = true }
//! ```
//!
//! Here is the example of the `rktk.json` file.
//! ```json
//! {
//!   "$schema": "https://raw.githubusercontent.com/nazo6/rktk/refs/heads/master/lib/rktk/schema.json",
//!   "constant": {
//!     "keyboard": {
//!       "cols": 12,
//!       "rows": 4,
//!       "right_rgb_count": 27,
//!       "left_rgb_count": 27
//!     }
//!   },
//!   "dynamic": {
//!     "keyboard": {
//!       "name": "corne",
//!     }
//!   }
//! }
//! ```
//!
//! The rktk json config is divided into two parts: const config and dynamic config.
//!
//! ## Constant config
//! The “const” config, as the name implies, is a config that sets constants and is used for values that need to be determined at compile time, such as buffer size.
//! This config can only be set by a json file set via `RKTK_CONFIG_PATH`, and the loaded values are in the [`CONST_CONFIG`].
//! The contents of config can be checked at [`schema::ConstantConfig`].
//!
//! ## Dynamic config
//! In contrast to const config, dynamic config is a config that can be configured at runtime.
//! Like const config, this config can be configured in a json file, but you can also define your own values in the rust code.
//!
//! This config can be used by putting it into the config property of the [`RktkOpts`] structure passed as the opts parameter of [`crate::task::start`].
//!
//! The dynamic config value obtained from the JSON file is assigned to [`DYNAMIC_CONFIG_FROM_FILE`].
//! The [`new_rktk_opts`] function can be used to initialize [`RktkOpts`] with this value,
//! but it is also possible to define your own RktkOpts by not using the JSON file value at all, or by editing some of it.
//!
//! For each configuration, see the [`schema::ConstantConfig`] and [`schema::DynamicConfig`] struct.
//!
//! # Storage
//!
//! In addition to the above programmatic config, config may also be loaded from storage.
//! Configuration is written to storage via rrp or other actions.
//! If the configuration is found in storage at startup, the above config will be overwritten with the contents of storage.

use crate::task::display::default_display::DefaultDisplayConfig;

include!(concat!(env!("OUT_DIR"), "/config.rs"));

pub mod keymap;
pub mod rgb;
pub mod storage;

/// rktk launch options.
///
/// Some properties of this struct requires `'static` lifetime. Even if the value you want to use
/// is not statically defined, you can obtain static lifetime using [`static_cell`] crate.
pub struct RktkOpts<D: crate::task::display::DisplayConfig, R: blinksy::layout::Layout2d> {
    /// Keymap of keyboard
    pub keymap: &'static keymap::Keymap,
    /// "dynamic" config
    pub config: &'static schema::DynamicConfig,
    /// Display layout
    pub display: D,
    pub rgb_layout: R,
    /// Hand of the keyboard. This is used for split keyboard. For non-split keyboard, set this
    /// value to `None`.
    ///
    /// If `None` is set, [`Hand::Left`] will be used.
    pub hand: Option<Hand>,
}

/// Creates a new [`RktkOpts`] with the default display config and the dynamic config loaded from
/// the JSON file.
pub fn new_rktk_opts(
    keymap: &'static keymap::Keymap,
    hand: Option<Hand>,
) -> RktkOpts<DefaultDisplayConfig, rgb::DummyLayout> {
    RktkOpts {
        keymap,
        config: &DYNAMIC_CONFIG_FROM_FILE,
        display: DefaultDisplayConfig,
        rgb_layout: rgb::DummyLayout,
        hand,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hand {
    Left,
    Right,
}

impl Hand {
    pub fn other(&self) -> Hand {
        match self {
            Hand::Left => Hand::Right,
            Hand::Right => Hand::Left,
        }
    }
}
