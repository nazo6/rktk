//! This module contains user-specific configurations.
//! I plan to allow users to customize these settings easily.
//!
//! TODO: Implement a way to change these settings.

#![allow(dead_code)]

use embassy_time::Duration;

/// 分割キーボード間通信速度
pub const SPLIT_BITRATE: f64 = 100000.0;
pub const SPLIT_CLK_DIVIDER: f64 = 125_000_000.0 / (SPLIT_BITRATE * 8.0);
pub const SPLIT_CHANNEL_SIZE: usize = 64;

pub const LEFT_DETECT_JUMPER_KEY: (usize, usize) = (2, 6);

/// Backlight led count for right side
pub const RIGHT_LED_NUM: usize = 34;
/// Backlight led count for left side. This is also used for non-split keyboard.
pub const LEFT_LED_NUM: usize = 37;

pub const USB_POLL_INTERVAL_KEYBOARD: u8 = 5;
pub const USB_POLL_INTERVAL_MOUSE: u8 = 5;

/// Time (msec) to wait for the next scan
pub const MIN_KB_SCAN_INTERVAL: Duration = Duration::from_millis(5);
pub const MIN_MOUSE_SCAN_INTERVAL: Duration = Duration::from_millis(5);

pub const DOUBLE_TAP_THRESHOLD: Duration = Duration::from_millis(500);

/// Time to wait for USB connection to determine master/slave
pub const SPLIT_USB_TIMEOUT: Duration = Duration::from_millis(200);

pub const AUTO_MOUSE_DURATION: Duration = Duration::from_millis(500);
/// When auto mouse mode is enabled, this layer is used
pub const AUTO_MOUSE_LAYER: usize = 1;
/// Mouse movement threshold to enable auto mouse mode
pub const AUTO_MOUSE_THRESHOLD: u8 = 1;

pub const DEFAULT_CPI: u16 = 400;

/// Divide mouse movement by this value
pub const SCROLL_DIVIDER_X: i8 = 20;
/// Divide mouse movement by this value. If this value is set to positive, it behaves like mac scrolling.
pub const SCROLL_DIVIDER_Y: i8 = -12;

pub const TAP_THRESHOLD: Duration = Duration::from_millis(200);
