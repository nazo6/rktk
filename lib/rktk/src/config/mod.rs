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

mod const_config;
pub use const_config::*;
use embassy_time::Duration;
use rktk_keymanager::state::config::{
    ComboConfig, KeyResolverConfig, MouseConfig, TapDanceConfig, TapHoldConfig,
};

#[derive(Clone, Default)]
pub struct Config {
    pub keyboard: Keyboard,
    pub rktk: RktkConfig,
    pub key_manager: KeyManagerConfig,
}

#[derive(smart_default::SmartDefault, Clone)]
pub struct Keyboard {
    /// The name of the keyboard.
    pub name: &'static str,

    /// Defines the layout of the keyboard used in the remapper.
    ///
    /// This is a JSON object that represents the layout of the keyboard and compatible with via's
    /// json layout format.
    pub layout: &'static str,

    /// The number of columns in the keyboard matrix.
    pub cols: u8,

    /// The number of rows in the keyboard matrix.
    pub rows: u8,

    /// A number representing the row number that the right col starts on in a split keyboard.
    ///
    /// If not set, `cols / 2` will be automatically set,
    /// so there is no need to set it if the number of columns on the right and left sides is the same.
    /// Also, there is no need to set it in the case of a non-split keyboard, as it is not used.
    pub split_right_shift: Option<u8>,

    /// The number of encoder keys.
    #[default(0)]
    pub encoder_count: u8,

    /// RGB led count for right side
    #[default(0)]
    pub right_led_count: usize,

    /// RGB led count for left side. This is also used for non-split keyboard.
    #[default(0)]
    pub left_led_count: usize,
}

#[derive(smart_default::SmartDefault, Clone)]
pub struct RktkConfig {
    /// The number of layers in the keyboard.
    #[default(5)]
    pub layer_count: u8,

    /// Threshold for double tap (ms).
    #[default(500)]
    pub double_tap_threshold: u64,

    /// Default CPI value for mouse
    #[default(600)]
    pub default_cpi: u16,

    /// Default duration of auto mouse mode (ms)
    #[default(500)]
    pub default_auto_mouse_duration: u32,

    /// Timeout for detecting split USB connection (ms).
    #[default(1000)]
    pub split_usb_timeout: u64,

    /// Time (ms) to wait for the next keyboard scan
    #[default(Duration::from_millis(5))]
    pub scan_interval_keyboard: Duration,

    /// Time (ms) to wait for the next mouse scan
    #[default(Duration::from_millis(5))]
    pub scan_interval_mouse: Duration,

    /// The size of the split channel. Usually, you don't need to change this value.
    #[default(64)]
    pub split_channel_size: usize,
}

#[derive(Clone)]
pub struct KeyManagerConfig {
    pub mouse: MouseConfig,
    pub key_resolver: KeyResolverConfig,
}

impl Default for KeyManagerConfig {
    fn default() -> Self {
        Self {
            mouse: MouseConfig {
                auto_mouse_layer: 1,
                auto_mouse_duration: 500,
                auto_mouse_threshold: 1,
                scroll_divider_x: 20,
                scroll_divider_y: -12,
            },
            key_resolver: KeyResolverConfig {
                tap_hold: TapHoldConfig {
                    threshold: 500,
                    hold_on_other_key: true,
                },
                tap_dance: TapDanceConfig { threshold: 100 },
                combo: ComboConfig { threshold: 20 },
            },
        }
    }
}
