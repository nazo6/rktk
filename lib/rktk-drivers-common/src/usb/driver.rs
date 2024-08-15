use super::{
    RemoteWakeupSignal, HID_KEYBOARD_CHANNEL, HID_MEDIA_KEYBOARD_CHANNEL, HID_MOUSE_CHANNEL,
};
use rktk::interface::{reporter::ReporterDriver, usb::UsbDriver};

pub struct CommonUsbDriver {
    pub(super) wakeup_signal: &'static RemoteWakeupSignal,
}

impl ReporterDriver for CommonUsbDriver {
    fn send_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), rktk::interface::error::RktkError> {
        HID_KEYBOARD_CHANNEL.try_send(_report);
        Ok(())
    }

    fn send_media_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), rktk::interface::error::RktkError> {
        HID_MEDIA_KEYBOARD_CHANNEL.try_send(_report);
        Ok(())
    }

    fn send_mouse_report(
        &self,
        _report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), rktk::interface::error::RktkError> {
        HID_MOUSE_CHANNEL.try_send(_report);
        Ok(())
    }

    fn send_rrp_data(&self, _data: &[u8]) -> Result<(), rktk::interface::error::RktkError> {
        Err(rktk::interface::error::RktkError::NotSupported)
    }

    async fn read_rrp_data(
        &self,
        _buf: &mut [u8],
    ) -> Result<(), rktk::interface::error::RktkError> {
        Err(rktk::interface::error::RktkError::NotSupported)
    }

    fn wakeup(&mut self) -> Result<(), rktk::interface::error::RktkError> {
        self.wakeup_signal.signal(());
        Ok(())
    }
}
impl UsbDriver for CommonUsbDriver {}
