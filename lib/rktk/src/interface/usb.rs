use super::error::RktkError;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

#[derive(Debug)]
pub enum HidReport {
    Keyboard(KeyboardReport),
    MediaKeyboard(MediaKeyboardReport),
    Mouse(MouseReport),
}

pub trait UsbDriver {
    /// Initialize the USB device and wait for ready.
    /// On devices that USB VBus detection is unavailable, this function is used to detect the
    /// master/slave.
    async fn wait_ready(&mut self);
    async fn send_report(&mut self, report: HidReport) -> Result<(), RktkError>;
    async fn wakeup(&mut self) -> Result<(), RktkError>;
}

pub enum DummyUsbDriver {}
impl UsbDriver for DummyUsbDriver {
    async fn wait_ready(&mut self) {}
    async fn send_report(&mut self, _report: HidReport) -> Result<(), RktkError> {
        Ok(())
    }
    async fn wakeup(&mut self) -> Result<(), RktkError> {
        Ok(())
    }
}
