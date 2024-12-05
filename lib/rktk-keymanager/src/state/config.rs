use crate::macros::common_derive;
use macro_rules_attribute::apply;

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
    pub tap_hold: TapHoldConfig,
    pub tap_dance: TapDanceConfig,
    pub combo: ComboConfig,
}

#[apply(common_derive)]
pub struct TapHoldConfig {
    pub threshold: u32,
    pub hold_on_other_key: bool,
}

#[apply(common_derive)]
pub struct TapDanceConfig {
    pub threshold: u32,
}

#[apply(common_derive)]
pub struct ComboConfig {
    pub threshold: u32,
}

#[apply(common_derive)]
pub struct KeymapInfo {
    pub layer_count: u8,
    pub max_tap_dance_key_count: u8,
    pub max_tap_dance_repeat_count: u8,
    pub oneshot_state_size: u8,
}
