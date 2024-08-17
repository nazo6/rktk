use super::{
    task::{
        HID_KEYBOARD_CHANNEL, HID_MEDIA_KEYBOARD_CHANNEL, HID_MOUSE_CHANNEL, RRP_RECV_PIPE,
        RRP_SEND_PIPE,
    },
    RemoteWakeupSignal,
};
use rktk::interface::{reporter::ReporterDriver, usb::UsbDriver};

pub struct CommonUsbDriver {
    pub(super) wakeup_signal: &'static RemoteWakeupSignal,
}

impl ReporterDriver for CommonUsbDriver {
    fn try_send_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), rktk::interface::error::RktkError> {
        HID_KEYBOARD_CHANNEL.try_send(_report);
        Ok(())
    }

    fn try_send_media_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), rktk::interface::error::RktkError> {
        HID_MEDIA_KEYBOARD_CHANNEL.try_send(_report);
        Ok(())
    }

    fn try_send_mouse_report(
        &self,
        _report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), rktk::interface::error::RktkError> {
        HID_MOUSE_CHANNEL.try_send(_report);
        Ok(())
    }

    fn try_send_rrp_data(&self, data: &[u8]) -> Result<(), rktk::interface::error::RktkError> {
        let mut wrote = 0;
        loop {
            let Ok(crr_wrote) = RRP_SEND_PIPE.try_write(&data[wrote..]) else {
                return Err(rktk::interface::error::RktkError::GeneralError(
                    "rrp_send_pipe full",
                ));
            };
            wrote += crr_wrote;
            if wrote == data.len() {
                break;
            }
        }
        Ok(())
    }
    async fn send_rrp_data(&self, data: &[u8]) -> Result<(), rktk::interface::error::RktkError> {
        RRP_SEND_PIPE.write_all(data).await;
        Ok(())
    }

    async fn read_rrp_data(
        &self,
        buf: &mut [u8],
    ) -> Result<usize, rktk::interface::error::RktkError> {
        let read = RRP_RECV_PIPE.read(buf).await;
        Ok(read)
    }

    fn wakeup(&self) -> Result<(), rktk::interface::error::RktkError> {
        if super::SUSPENDED.load(core::sync::atomic::Ordering::SeqCst) {
            self.wakeup_signal.signal(());
            return Ok(());
        }
        Ok(())
    }
}
impl UsbDriver for CommonUsbDriver {}
