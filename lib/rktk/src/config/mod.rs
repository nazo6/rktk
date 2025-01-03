//! Static configuration of the firmware.
//!
//! There are two types of static config.
//! The first one is [`Config`] struct. You can make this value in code and pass to [`crate::task::start`] function.
//! The second one is [`ConstConfig`] struct. This is read from environment variables at compile time. You can use `.cargo/config.toml` to set these values.

mod const_config;
pub use const_config::*;
use embassy_time::Duration;
use rktk_keymanager::config::{
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
}

#[derive(smart_default::SmartDefault, Clone)]
pub struct RktkConfig {
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
