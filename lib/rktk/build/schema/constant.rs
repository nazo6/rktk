use smart_default::SmartDefault;

/// Root struct of the "constant" config
#[macro_rules_attribute::apply(crate::schema::common_derive)]
pub struct ConstantConfig {
    pub keyboard: KeyboardConstantConfig,
    #[serde(default)]
    pub buffer: BufferSizeConfig,
    #[serde(default)]
    pub key_manager: KeymanagerConstantConfig,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
pub struct KeyboardConstantConfig {
    /// The number of columns in the keyboard matrix.
    pub cols: u8,

    /// The number of rows in the keyboard matrix.
    pub rows: u8,

    /// The number of encoder keys.
    #[serde(default)]
    pub encoder_count: u8,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(SmartDefault)]
#[serde(default)]
pub struct BufferSizeConfig {
    /// Size of the buffer used by rrp
    #[default(512)]
    pub rrp: usize,

    /// Size of the log channel buffer
    #[default(64)]
    pub log_channel: usize,

    /// Size of the split channel buffer
    #[default(64)]
    pub split_channel: usize,

    /// Size of the mouse event buffer
    #[default(4)]
    pub mouse_event: usize,

    /// Size of the keyboard event buffer
    #[default(4)]
    pub keyboard_event: usize,

    /// Size of the encoder event buffer
    #[default(4)]
    pub encoder_event: usize,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(SmartDefault)]
#[serde(default)]
pub struct KeymanagerConstantConfig {
    #[default(5)]
    pub layer_count: u8,

    #[default(8)]
    pub normal_max_pressed_keys: usize,

    #[default(4)]
    pub oneshot_state_size: usize,

    #[default(2)]
    pub tap_dance_max_definitions: usize,

    #[default(4)]
    pub tap_dance_max_repeats: usize,

    #[default(2)]
    pub combo_key_max_definitions: usize,

    #[default(3)]
    pub combo_key_max_sources: usize,
}
