#[derive(
    serde::Deserialize, schemars::JsonSchema, const_gen::CompileConst, smart_default::SmartDefault,
)]
#[inherit_doc]
#[serde(default)]
pub struct RktkConfig {
    /// The number of layers in the keyboard.
    #[default(5)]
    pub layer_count: u8,

    /// Threshold for double tap (ms).
    #[default(500)]
    pub double_tap_threshold: u64,

    /// Default CPI value for mouse
    #[default(600)]
    pub default_cpi: u16,

    /// Default duration of auto mouse mode (ms)
    #[default(500)]
    pub default_auto_mouse_duration: u32,

    /// Timeout for detecting split USB connection (ms).
    #[default(1000)]
    pub split_usb_timeout: u64,

    /// Time (ms) to wait for the next keyboard scan
    #[default(5)]
    pub scan_interval_keyboard: u64,

    /// Time (ms) to wait for the next mouse scan
    #[default(5)]
    pub scan_interval_mouse: u64,

    /// The size of the split channel. Usually, you don't need to change this value.
    #[default(64)]
    pub split_channel_size: usize,

    #[default(64)]
    pub log_channel_size: usize,

    #[default(4)]
    pub mouse_event_buffer_size: usize,

    #[default(4)]
    pub keyboard_event_buffer_size: usize,

    #[default(4)]
    pub encoder_event_buffer_size: usize,
}
