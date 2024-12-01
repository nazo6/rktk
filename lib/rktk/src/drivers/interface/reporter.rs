use super::error::RktkError;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

pub trait ReporterDriver {
    async fn wait_ready(&self) {}
    fn try_send_keyboard_report(&self, _report: KeyboardReport) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
    fn try_send_media_keyboard_report(
        &self,
        _report: MediaKeyboardReport,
    ) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
    fn try_send_mouse_report(&self, _report: MouseReport) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }

    fn try_send_rrp_data(&self, _data: &[u8]) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
    async fn read_rrp_data(&self, _buf: &mut [u8]) -> Result<usize, RktkError> {
        let _: () = core::future::pending().await;
        Err(RktkError::NotSupported)
    }

    fn wakeup(&self) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
}