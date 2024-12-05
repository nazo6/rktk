use crate::macros::common_derive;
use macro_rules_attribute::apply;

use crate::keycode::KeyCode;

/// Configuration to initialize the keyboard state.
#[apply(common_derive)]
pub struct StateConfig {
    pub mouse: MouseConfig,
    pub key_resolver: KeyResolverConfig,
    pub initial_output: Output,
}

#[apply(common_derive)]
#[derive(Copy)]
pub enum Output {
    Usb,
    Ble,
}

#[apply(common_derive)]
pub struct MouseConfig {
    pub auto_mouse_layer: u8,
    pub auto_mouse_duration: u32,
    pub auto_mouse_threshold: u8,
    pub scroll_divider_x: i8,
    pub scroll_divider_y: i8,
}

#[apply(common_derive)]
pub struct KeyResolverConfig {
    pub tap_threshold: u32,
    pub tap_dance: TapDanceConfig,
}

#[apply(common_derive)]
pub struct TapDanceConfig {
    pub threshold: u32,
    pub definitions: [Option<TapDanceDefinition>; MAX_TAP_DANCE_KEY_COUNT as usize],
}

#[apply(common_derive)]
pub struct TapDanceDefinition {
    pub tap: [Option<KeyCode>; MAX_TAP_DANCE_REPEAT_COUNT as usize],
    pub hold: [Option<KeyCode>; MAX_TAP_DANCE_REPEAT_COUNT as usize],
}

#[apply(common_derive)]
pub struct KeymapInfo {
    pub layer_count: u8,
    pub max_tap_dance_key_count: u8,
    pub max_tap_dance_repeat_count: u8,
    pub oneshot_state_size: u8,
    pub max_resolved_key_count: u8,
}

pub(super) const MAX_TAP_DANCE_KEY_COUNT: u8 = 4;
pub(super) const MAX_TAP_DANCE_REPEAT_COUNT: u8 = 4;
pub(super) const ONESHOT_STATE_SIZE: u8 = 4;
pub(super) const MAX_RESOLVED_KEY_COUNT: u8 = 64;
