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

    fn wakeup(&self) -> Result<(), Self::Error>;
}
