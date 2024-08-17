#[cfg(not(no_build))]
macro_rules! def {
    ($name:tt, $type:ty) => {
        const fn $name<const U: $type>() -> $type {
            U
        }
    };
}

#[cfg(not(no_build))]
def!(u64_default, u64);
#[cfg(not(no_build))]
def!(usize_default, usize);
#[cfg(not(no_build))]
def!(i8_default, i8);
#[cfg(not(no_build))]
def!(u16_default, u16);
#[cfg(not(no_build))]
def!(u8_default, u8);

#[cfg_attr(
    not(no_build),
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct StaticConfig {
    /// The name of the keyboard.
    #[cfg(not(no_build))]
    pub name: String,
    #[cfg(no_build)]
    pub name: &'static str,

    /// The layout of the keyboard.
    #[cfg(not(no_build))]
    pub layout_json: String,
    #[cfg(no_build)]
    pub layout_json: &'static str,

    /// Timeout for detecting split USB connection (ms).
    #[cfg_attr(not(no_build), serde(default = "u64_default::<200>"))]
    pub split_usb_timeout: u64,

    /// Threshold for double tap (ms).
    #[cfg_attr(not(no_build), serde(default = "u64_default::<500>"))]
    pub double_tap_threshold: u64,

    /// Time (ms) to wait for the next keyboard scan
    #[cfg_attr(not(no_build), serde(default = "u64_default::<5>"))]
    pub scan_interval_keyboard: u64,

    /// Time (ms) to wait for the next mouse scan
    #[cfg_attr(not(no_build), serde(default = "u64_default::<5>"))]
    pub scan_interval_mouse: u64,

    /// The number of columns in the keyboard matrix.
    pub cols: usize,

    /// The number of rows in the keyboard matrix.
    pub rows: usize,

    /// The number of layers in the keyboard.
    #[cfg_attr(not(no_build), serde(default = "usize_default::<5>"))]
    pub layer_count: usize,

    /// Backlight led count for right side
    #[cfg_attr(not(no_build), serde(default = "usize_default::<0>"))]
    pub right_led_count: usize,

    /// Backlight led count for left side. This is also used for non-split keyboard.
    #[cfg_attr(not(no_build), serde(default = "usize_default::<0>"))]
    pub left_led_count: usize,

    /// The size of the split channel. Usually, you don't need to change this value.
    #[cfg_attr(not(no_build), serde(default = "usize_default::<64>"))]
    pub split_channel_size: usize,

    /// Default CPI value for mouse
    #[cfg_attr(not(no_build), serde(default = "u16_default::<600>"))]
    pub default_cpi: u16,

    /// Default duration of auto mouse mode (ms)
    #[cfg_attr(not(no_build), serde(default = "u64_default::<500>"))]
    pub default_auto_mouse_duration: u64,

    /// When auto mouse mode is enabled, this layer is used
    #[cfg_attr(not(no_build), serde(default = "usize_default::<1>"))]
    pub default_auto_mouse_layer: usize,

    /// Mouse movement threshold to enable auto mouse mode
    #[cfg_attr(not(no_build), serde(default = "u8_default::<1>"))]
    pub default_auto_mouse_threshold: u8,

    /// Scroll divider for x axis
    #[cfg_attr(not(no_build), serde(default = "i8_default::<20>"))]
    pub default_scroll_divider_x: i8,

    /// Scroll divider for y axis
    #[cfg_attr(not(no_build), serde(default = "i8_default::<-12>"))]
    pub default_scroll_divider_y: i8,

    /// Threshold for tap (ms)
    #[cfg_attr(not(no_build), serde(default = "u64_default::<200>"))]
    pub default_tap_threshold: u64,
}
