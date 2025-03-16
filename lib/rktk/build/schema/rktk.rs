#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(smart_default::SmartDefault)]
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

    /// Swap the x and y values obtained from the mouse driver. This also affects the scroll direction.
    #[default(false)]
    pub swap_mouse_x_y: bool,

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

    /// Size of the buffer used by rrp
    #[default(512)]
    pub rrp_buffer_size: usize,

    /// Size of the split channel buffer
    #[default(64)]
    pub split_channel_size: usize,

    /// Size of the log channel buffer
    #[default(64)]
    pub log_channel_size: usize,

    /// Size of the mouse event buffer
    #[default(4)]
    pub mouse_event_buffer_size: usize,

    /// Size of the keyboard event buffer
    #[default(4)]
    pub keyboard_event_buffer_size: usize,

    /// Size of the encoder event buffer
    #[default(4)]
    pub encoder_event_buffer_size: usize,

    /// Time(ms) until the display is turned off if there is no activity
    #[default(20000)]
    pub display_timeout: u64,

    /// rktk basically updates the keyboard state only when it receives an event from the hardware.
    /// However, many states are time-dependent and can change even without an event.
    /// Polling at regular time intervals is necessary to monitor such changes.
    ///
    /// This setting specifies that interval. (ms)
    #[default(10)]
    pub state_update_interval: u64,
}
