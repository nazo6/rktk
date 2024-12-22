use super::{
    task::{
        HID_KEYBOARD_CHANNEL, HID_MEDIA_KEYBOARD_CHANNEL, HID_MOUSE_CHANNEL, RRP_RECV_PIPE,
        RRP_SEND_PIPE,
    },
    ReadySignal, RemoteWakeupSignal,
};
use rktk::drivers::interface::{reporter::ReporterDriver, usb::UsbDriver};

pub struct CommonUsbDriver {
    pub(super) wakeup_signal: &'static RemoteWakeupSignal,
    pub(super) ready_signal: &'static ReadySignal,
}

#[derive(Debug, thiserror::Error)]
pub enum UsbError {
    #[error("Channel full: {0}")]
    ChannelFull(&'static str),
    #[error("Not supported")]
    NotSupported,
}

impl ReporterDriver for CommonUsbDriver {
    type Error = UsbError;

    async fn wait_ready(&self) {
        self.ready_signal.wait().await;
    }

    fn try_send_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        HID_KEYBOARD_CHANNEL
            .try_send(_report)
            .map_err(|_| UsbError::ChannelFull("keyboard"))?;
        Ok(())
    }

    fn try_send_media_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        HID_MEDIA_KEYBOARD_CHANNEL
            .try_send(_report)
            .map_err(|_| UsbError::ChannelFull("media keyboard"))?;

        Ok(())
    }

    fn try_send_mouse_report(
        &self,
        _report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        HID_MOUSE_CHANNEL
            .try_send(_report)
            .map_err(|_| UsbError::ChannelFull("mouse"))?;

        Ok(())
    }

    async fn send_rrp_data(&self, data: &[u8]) -> Result<(), Self::Error> {
        RRP_SEND_PIPE.write_all(data).await;
        Ok(())
    }

    async fn read_rrp_data(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let read = RRP_RECV_PIPE.read(buf).await;
        Ok(read)
    }

    fn wakeup(&self) -> Result<(), Self::Error> {
        if super::SUSPENDED.load(core::sync::atomic::Ordering::Acquire) {
            self.wakeup_signal.signal(());
            return Ok(());
        }
        Ok(())
    }
}
impl UsbDriver for CommonUsbDriver {
    type Error = UsbError;

    async fn vbus_detect(&self) -> Result<bool, <Self as UsbDriver>::Error> {
        Err(UsbError::NotSupported)
    }
}
