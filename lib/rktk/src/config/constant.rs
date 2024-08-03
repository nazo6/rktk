//! Configs that must be set at compile time.

use embassy_time::Duration;
use konst::{option::unwrap_or, primitive::parse_usize, unwrap_ctx};

/// Time to wait for USB connection to determine master/slave
pub const SPLIT_USB_TIMEOUT: Duration = Duration::from_millis(200);

pub const DOUBLE_TAP_THRESHOLD: Duration = Duration::from_millis(500);

/// Time (msec) to wait for the next scan
pub const SCAN_INTERVAL_KEYBOARD: Duration = Duration::from_millis(5);
pub const SCAN_INTEVAL_MOUSE: Duration = Duration::from_millis(5);

/// The number of columns in the keyboard matrix.
/// Env key: `RKTK_COLS`
pub const COLS: usize = unwrap_ctx!(parse_usize(env!("RKTK_COLS")));
/// The number of rows in the keyboard matrix.
/// Env key: `RKTK_ROWS`
pub const ROWS: usize = unwrap_ctx!(parse_usize(env!("RKTK_ROWS")));

/// The number of layers in the keyboard.
/// Env key: `RKTK_LAYER_COUNT`
///
/// Making this value larger may cause memory overflow.
pub const LAYER_COUNT: usize = unwrap_ctx!(parse_usize(env!("RKTK_LAYER_COUNT")));

/// Backlight led count for right side
pub const RIGHT_LED_COUNT: usize = unwrap_ctx!(parse_usize(unwrap_or!(
    option_env!("RKTK_RIGHT_LED_COUNT"),
    "0"
)));
/// Backlight led count for left side. This is also used for non-split keyboard.
pub const LEFT_LED_COUNT: usize = unwrap_ctx!(parse_usize(unwrap_or!(
    option_env!("RKTK_LEFT_LED_COUNT"),
    "0"
)));

pub const SPLIT_CHANNEL_SIZE: usize = 64;

pub const DEFAULT_CPI: u16 = 400;

pub const DEFAULT_AUTO_MOUSE_DURATION: Duration = Duration::from_millis(500);
/// When auto mouse mode is enabled, this layer is used
pub const DEFAULT_AUTO_MOUSE_LAYER: usize = 1;
/// Mouse movement threshold to enable auto mouse mode
pub const DEFAULT_AUTO_MOUSE_THRESHOLD: u8 = 1;

/// Divide mouse movement by this value
pub const DEFAULT_SCROLL_DIVIDER_X: i8 = 20;
/// Divide mouse movement by this value. If this value is set to positive, it behaves like mac scrolling.
pub const DEFAULT_SCROLL_DIVIDER_Y: i8 = -12;

pub const DEFAULT_TAP_THRESHOLD: Duration = Duration::from_millis(200);
