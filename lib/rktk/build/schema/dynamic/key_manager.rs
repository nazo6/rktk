// Private struct available in this module is copied from rktk_keymanager::interface::state::config::* to avoid foreign impl problem.
// `use keymanager::...::config::*` is inserted in top of generated code and types in this module
// will not be used.

use smart_default::SmartDefault;

#[doc = r#"
Config for key manager.

Note that these values are "default value" expect for `constant` fields. If storage is enabled in your firmware, these
values can be overwritten by the values stored in the storage.
"#]
#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(Default)]
#[serde(default)]
pub struct KeyManagerConfig {
    pub mouse: MouseConfig,
    pub key_resolver: KeyResolverConfig,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(SmartDefault)]
#[serde(default)]
struct MouseConfig {
    #[default(1)]
    pub auto_mouse_layer: u8,

    #[default(500)]
    pub auto_mouse_duration: u32,

    #[default(0)]
    pub auto_mouse_threshold: u8,

    #[default(20)]
    pub scroll_divider_x: i8,

    #[default(-12)]
    pub scroll_divider_y: i8,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(SmartDefault)]
#[serde(default)]
struct KeyResolverConfig {
    pub tap_hold: TapHoldConfig,
    pub tap_dance: TapDanceConfig,
    pub combo: ComboConfig,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(SmartDefault)]
#[serde(default)]
struct TapHoldConfig {
    #[default(200)]
    pub threshold: u32,

    #[default(true)]
    pub hold_on_other_key: bool,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(SmartDefault)]
#[serde(default)]
struct TapDanceConfig {
    #[default(200)]
    pub threshold: u32,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(SmartDefault)]
#[serde(default)]
struct ComboConfig {
    #[default(50)]
    pub threshold: u32,
}
