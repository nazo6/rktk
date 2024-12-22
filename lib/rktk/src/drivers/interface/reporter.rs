use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

pub trait ReporterDriver {
    type Error: core::error::Error;

    async fn wait_ready(&self) {}
    fn try_send_keyboard_report(&self, _report: KeyboardReport) -> Result<(), Self::Error>;

    fn try_send_media_keyboard_report(
        &self,
        _report: MediaKeyboardReport,
    ) -> Result<(), Self::Error>;
    fn try_send_mouse_report(&self, _report: MouseReport) -> Result<(), Self::Error>;

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error>;
    async fn read_rrp_data(&self, _buf: &mut [u8]) -> Result<usize, Self::Error> {
        let _: () = core::future::pending().await;
        Ok(0)
    }

    /// Wake up the device.
    /// This is used to wake up the device from suspend mode.
    ///
    /// # Returns
    /// - `Ok(true)`: Woke up signal sent successfully.
    /// - `Ok(false)`: The device is already awake.
    /// - `Err(_)`: Failed to send the wake up signal or not supported.
    fn wakeup(&self) -> Result<bool, Self::Error>;
}
