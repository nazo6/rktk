#[cfg(not(no_build))]
mod default_val {
    macro_rules! def {
        ($name:tt, $type:ty) => {
            pub const fn $name<const U: $type>() -> $type {
                U
            }
        };
    }

    def!(u64_default, u64);
    def!(u32_default, u32);
    def!(usize_default, usize);
    def!(i8_default, i8);
    def!(u16_default, u16);
    def!(u8_default, u8);
}

#[cfg(not(no_build))]
use default_val::*;

#[cfg_attr(
    not(no_build),
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct StaticConfig {
    pub keyboard: Keyboard,
    pub config: Config,
}

// layout is json data in config file. but in keyboard, it's just a string.
#[cfg(not(no_build))]
fn serialize_layout<S: serde::Serializer>(
    val: &serde_json::Value,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&serde_json::to_string(val).unwrap())
}

/// Keyboard information
#[cfg_attr(
    not(no_build),
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct Keyboard {
    /// The name of the keyboard.
    #[cfg(not(no_build))]
    pub name: String,
    #[cfg(no_build)]
    pub name: &'static str,

    /// The layout of the keyboard.
    #[cfg(not(no_build))]
    #[cfg_attr(not(no_build), serde(serialize_with = "serialize_layout"))]
    pub layout: serde_json::Value,
    #[cfg(no_build)]
    pub layout: &'static str,

    /// The number of columns in the keyboard matrix.
    pub cols: u8,

    /// The number of rows in the keyboard matrix.
    pub rows: u8,

    /// Backlight led count for right side
    #[cfg_attr(not(no_build), serde(default = "usize_default::<0>"))]
    pub right_led_count: usize,

    /// Backlight led count for left side. This is also used for non-split keyboard.
    #[cfg_attr(not(no_build), serde(default = "usize_default::<0>"))]
    pub left_led_count: usize,
}

/// Configuration for the firmware.
#[cfg_attr(
    not(no_build),
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct Config {
    pub rktk: RktkConfig,
}

#[cfg_attr(
    not(no_build),
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct RktkConfig {
    /// The number of layers in the keyboard.
    #[cfg_attr(not(no_build), serde(default = "u8_default::<5>"))]
    pub layer_count: u8,

    /// Threshold for double tap (ms).
    #[cfg_attr(not(no_build), serde(default = "u64_default::<500>"))]
    pub double_tap_threshold: u64,

    /// Threshold for tap (ms)
    #[cfg_attr(not(no_build), serde(default = "u32_default::<200>"))]
    pub default_tap_threshold: u32,

    /// Threshold for tap dance (ms)
    #[cfg_attr(not(no_build), serde(default = "u32_default::<100>"))]
    pub default_tap_dance_threshold: u32,

    /// Default CPI value for mouse
    #[cfg_attr(not(no_build), serde(default = "u16_default::<600>"))]
    pub default_cpi: u16,

    /// Default duration of auto mouse mode (ms)
    #[cfg_attr(not(no_build), serde(default = "u32_default::<500>"))]
    pub default_auto_mouse_duration: u32,

    /// When auto mouse mode is enabled, this layer is used
    #[cfg_attr(not(no_build), serde(default = "u8_default::<1>"))]
    pub default_auto_mouse_layer: u8,

    /// Mouse movement threshold to enable auto mouse mode
    #[cfg_attr(not(no_build), serde(default = "u8_default::<1>"))]
    pub default_auto_mouse_threshold: u8,

    /// Scroll divider for x axis
    #[cfg_attr(not(no_build), serde(default = "i8_default::<20>"))]
    pub default_scroll_divider_x: i8,

    /// Scroll divider for y axis
    #[cfg_attr(not(no_build), serde(default = "i8_default::<-12>"))]
    pub default_scroll_divider_y: i8,

    /// Timeout for detecting split USB connection (ms).
    #[cfg_attr(not(no_build), serde(default = "u64_default::<600>"))]
    pub split_usb_timeout: u64,

    /// Time (ms) to wait for the next keyboard scan
    #[cfg_attr(not(no_build), serde(default = "u64_default::<5>"))]
    pub scan_interval_keyboard: u64,

    /// Time (ms) to wait for the next mouse scan
    #[cfg_attr(not(no_build), serde(default = "u64_default::<5>"))]
    pub scan_interval_mouse: u64,

    /// The size of the split channel. Usually, you don't need to change this value.
    #[cfg_attr(not(no_build), serde(default = "usize_default::<64>"))]
    pub split_channel_size: usize,
}
