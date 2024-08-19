use embassy_time::Duration;

use crate::keycode::KeyCode;

#[derive(Debug, Clone)]
pub struct StateConfig {
    pub mouse: MouseConfig,
    pub key_resolver: KeyResolverConfig,
}

#[derive(Debug, Clone)]
pub struct MouseConfig {
    pub auto_mouse_layer: usize,
    pub auto_mouse_duration: Duration,
    pub auto_mouse_threshold: u8,
    pub scroll_divider_x: i8,
    pub scroll_divider_y: i8,
}

#[derive(Debug, Clone)]
pub struct KeyResolverConfig {
    pub tap_threshold: Duration,
    pub tap_dash_threshold: Duration,
    pub tap_dance: [Option<TapDanceConfig>; MAX_TAP_DANCE_COUNT],
}

#[derive(Debug, Clone)]
pub struct TapDanceConfig {
    pub tap: [Option<KeyCode>; MAX_TAP_DANCE_KEY_COUNT],
    pub hold: [Option<KeyCode>; MAX_TAP_DANCE_KEY_COUNT],
}

pub(super) const MAX_TAP_DANCE_KEY_COUNT: usize = 4;
pub(super) const MAX_TAP_DANCE_COUNT: usize = 8;
pub(super) const MAX_ONESHOT_COUNT: usize = 4;
pub(super) const MAX_RESOLVED_KEY_COUNT: usize = 64;
