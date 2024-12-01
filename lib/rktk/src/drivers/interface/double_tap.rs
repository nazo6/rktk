use embassy_time::Duration;

/// Interface for double tap reset drivers.
pub trait DoubleTapResetDriver {
    /// Determine if a double-tap has been performed and reboot to the bootloader appropriately
    async fn execute(&self, timeout: Duration);
}
