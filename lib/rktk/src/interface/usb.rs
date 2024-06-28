use super::error::RktkError;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

pub enum HidReport {
    Keyboard(KeyboardReport),
    MediaKeyboard(MediaKeyboardReport),
    Mouse(MouseReport),
}

pub trait Usb {
    /// Initialize the USB device and wait for ready.
    /// On devices that USB VBus detection is unavailable, this function is used to detect the
    /// master/slave.
    async fn wait_ready(&mut self);
    async fn send_report(&mut self, report: HidReport) -> Result<(), RktkError>;
    async fn wakeup(&mut self) -> Result<(), RktkError>;
}
