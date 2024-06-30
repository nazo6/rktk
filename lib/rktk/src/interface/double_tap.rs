use embassy_time::Duration;

/// Interface for double tap reset drivers.
pub trait DoubleTapResetDriver {
    /// Determine if a double-tap has been performed and reboot to the bootloader appropriately
    async fn execute(&self, timeout: Duration);
}

/// Dummy driver that is only used to be given as a type argument.
pub enum DummyDoubleTapResetDriver {}
impl DoubleTapResetDriver for DummyDoubleTapResetDriver {
    async fn execute(&self, _timeout: Duration) {}
}
