use super::{
    ReadySignal,
    raw_hid::RAW_HID_BUFFER_SIZE,
    task::{
        HID_KEYBOARD_CHANNEL, HID_MEDIA_KEYBOARD_CHANNEL, HID_MOUSE_CHANNEL, KEYBOARD_LED_SIGNAL,
        RAW_HID_SEND_CHANNEL, RRP_RECV_PIPE, RRP_SEND_PIPE,
    },
};
use rktk::drivers::interface::{reporter::ReporterDriver, usb::UsbReporterDriver};

pub struct CommonUsbDriver {
    #[cfg(feature = "usb-remote-wakeup")]
    pub(super) wakeup_signal: &'static super::RemoteWakeupSignal,
    pub(super) ready_signal: &'static ReadySignal,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbError {
    ChannelFull(&'static str),
    NotSupported,
    TooBig,
    BufSmall,
}

impl rktk::drivers::interface::Error for UsbError {}

impl ReporterDriver for CommonUsbDriver {
    type Error = UsbError;

    async fn wait_ready(&self) {
        self.ready_signal.wait().await;
    }

    async fn recv_keyboard_report(&self) -> Result<u8, Self::Error> {
        Ok(KEYBOARD_LED_SIGNAL.wait().await)
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

    async fn recv_rrp_data(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let read = RRP_RECV_PIPE.read(buf).await;
        Ok(read)
    }

    async fn send_raw_hid_data(&self, data: &[u8]) -> Result<(), Self::Error> {
        if data.len() > RAW_HID_BUFFER_SIZE {
            return Err(Self::Error::TooBig);
        }

        let mut buf = [0; RAW_HID_BUFFER_SIZE];
        buf[0..data.len()].copy_from_slice(data);

        RAW_HID_SEND_CHANNEL.send(buf).await;
        Ok(())
    }

    async fn recv_raw_hid_data(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.len() < RAW_HID_BUFFER_SIZE {
            return Err(Self::Error::BufSmall);
        }

        let data = RAW_HID_SEND_CHANNEL.receive().await;
        buf[0..data.len()].copy_from_slice(&data);

        Ok(data.len())
    }

    fn wakeup(&self) -> Result<bool, Self::Error> {
        #[cfg(feature = "usb-remote-wakeup")]
        {
            if super::SUSPENDED.load(core::sync::atomic::Ordering::Acquire) {
                self.wakeup_signal.signal(());
                return Ok(true);
            }
            Ok(false)
        }

        #[cfg(not(feature = "usb-remote-wakeup"))]
        Err(UsbError::NotSupported)
    }
}
impl UsbReporterDriver for CommonUsbDriver {
    type Error = UsbError;

    async fn vbus_detect(&self) {
        super::POWERED_SIGNAL.wait().await;
    }
}
