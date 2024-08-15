use super::error::RktkError;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

pub trait ReporterDriver {
    fn send_keyboard_report(&self, _report: KeyboardReport) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
    fn send_media_keyboard_report(&self, _report: MediaKeyboardReport) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
    fn send_mouse_report(&self, _report: MouseReport) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }

    fn send_rrp_data(&self, _data: &[u8]) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
    async fn read_rrp_data(&self, _buf: &mut [u8]) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }

    fn wakeup(&mut self) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
}

pub enum DummyReporterDriver {}
impl ReporterDriver for DummyReporterDriver {}
