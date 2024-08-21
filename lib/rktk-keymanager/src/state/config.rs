use core::time::Duration;

use crate::keycode::KeyCode;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone)]
pub struct StateConfig {
    pub mouse: MouseConfig,
    pub key_resolver: KeyResolverConfig,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone)]
pub struct MouseConfig {
    pub auto_mouse_layer: u8,
    pub auto_mouse_duration: u32,
    pub auto_mouse_threshold: u8,
    pub scroll_divider_x: i8,
    pub scroll_divider_y: i8,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone)]
pub struct KeyResolverConfig {
    pub tap_threshold: u32,
    pub tap_dance_threshold: u32,
    pub tap_dance: [Option<TapDanceConfig>; MAX_TAP_DANCE_REPEAT_COUNT as usize],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone)]
pub struct TapDanceConfig {
    pub tap: [Option<KeyCode>; MAX_TAP_DANCE_KEY_COUNT as usize],
    pub hold: [Option<KeyCode>; MAX_TAP_DANCE_KEY_COUNT as usize],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Debug, Clone)]
pub struct KeymapInfo {
    pub layer_count: u8,
    pub max_tap_dance_key_count: u8,
    pub max_tap_dance_repeat_count: u8,
    pub oneshot_state_size: u8,
    pub max_resolved_key_count: u8,
}

pub(super) const MAX_TAP_DANCE_KEY_COUNT: u8 = 4;
pub(super) const MAX_TAP_DANCE_REPEAT_COUNT: u8 = 8;
pub(super) const ONESHOT_STATE_SIZE: u8 = 4;
pub(super) const MAX_RESOLVED_KEY_COUNT: u8 = 64;
