/// RKTK behavior config
#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(smart_default::SmartDefault)]
#[serde(default)]
pub struct RktkConfig {
    pub rgb: RktkRgbConfig,
    pub role_detection: RktkRoleDetectionConfig,

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

    /// Time (ms) to wait for the next keyboard scan
    #[default(5)]
    pub scan_interval_keyboard: u64,

    /// Time (ms) to wait for the next mouse scan
    #[default(5)]
    pub scan_interval_mouse: u64,

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

/// RKTK RGB config
#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(smart_default::SmartDefault)]
#[serde(default)]
pub struct RktkRgbConfig {
    /// Time(ms) to wait for the next RGB pattern update
    ///
    /// Lower values will result in smoother animations, but may increase power consumption.
    /// Also, for heavy patterns, it may cause the MCU to be busy for a long time, which affects
    /// mouse latency (especially).
    #[default(16)]
    pub pattern_update_interval: u64,

    /// Initial RGB blightness
    ///
    /// Range: 0.0 to 1.0
    #[default(0.5)]
    pub default_brightness: f32,
}

/// RKTK role detection config
#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(smart_default::SmartDefault)]
#[serde(default)]
pub struct RktkRoleDetectionConfig {
    /// Timeout for detecting split USB connection (ms).
    #[default(1000)]
    pub usb_timeout: u64,

    pub timeout_behavior: RktkRoleDetectionTimeoutBehavior,

    /// Method for role detection
    pub method: RktkRoleDetectionMethod,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(smart_default::SmartDefault)]
pub enum RktkRoleDetectionMethod {
    #[default]
    Auto,
    ForceMaster,
    ForceSlave,
}

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(smart_default::SmartDefault)]
pub enum RktkRoleDetectionTimeoutBehavior {
    #[default]
    None,
    ForceMaster,
    ForceSlave,
}
