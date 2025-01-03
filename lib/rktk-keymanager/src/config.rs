use crate::macros::common_derive;
use konst::{option::unwrap_or, primitive::parse_u8, unwrap_ctx};
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

macro_rules! config_usize {
    ($env:literal, $def:literal) => {
        unwrap_ctx!(parse_u8(unwrap_or!(option_env!($env), $def)))
    };
}

/// The part of config that must be known at compile time.
///
/// These config values are read from environment variables at compile time.
pub struct ConstConfig {
    /// Defines how many one-shot keys can be active at the same time.
    ///
    /// default: `8`
    /// env: `RKTK_KM_ONE_SHOT_STATE_SIZE`
    pub one_shot_state_size: u8,

    /// Defines how many keys can be used in a tap dance definition.
    ///
    /// default: `4`
    /// env: `RKTK_KM_MAX_TAP_DANCE_KEY_COUNT`
    pub max_tap_dance_key_count: u8,

    /// Defines how many times a tap dance key can be repeated.
    ///
    /// default: `4`
    /// env: `RKTK_KM_MAX_TAP_DANCE_REPEAT_COUNT`
    pub max_tap_dance_repeat_count: u8,

    /// Defines how many keys can be used as source keys in a combo definition.
    ///
    /// default: `3`
    /// env: `RKTK_KM_MAX_COMBO_COMBINATION_COUNT`
    pub max_combo_combination_count: u8,

    /// Defines how many combos are defined.
    ///
    /// default: `4`
    /// env: `RKTK_KM_MAX_COMBO_KEY_COUNT`
    pub max_combo_key_count: u8,
}

pub const CONST_CONFIG: ConstConfig = ConstConfig {
    one_shot_state_size: config_usize!("RKTK_KM_ONE_SHOT_STATE_SIZE", "8"),
    max_tap_dance_key_count: config_usize!("RKTK_KM_MAX_TAP_DANCE_KEY_COUNT", "4"),
    max_tap_dance_repeat_count: config_usize!("RKTK_KM_MAX_TAP_DANCE_REPEAT_COUNT", "4"),
    max_combo_combination_count: config_usize!("RKTK_KM_MAX_COMBO_COMBINATION_COUNT", "3"),
    max_combo_key_count: config_usize!("RKTK_KM_MAX_COMBO_KEY_COUNT", "4"),
};
