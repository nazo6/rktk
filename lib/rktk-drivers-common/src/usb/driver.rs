use embassy_usb::class::hid::HidReaderWriter;
use embassy_usb::driver::Driver;

use super::{RemoteWakeupSignal, SUSPENDED};
use rktk::interface::usb::UsbDriver;

pub struct HidReaderWriters<'a, D: Driver<'a>> {
    pub keyboard: HidReaderWriter<'a, D, 1, 8>,
    pub mouse: HidReaderWriter<'a, D, 1, 8>,
    pub media_key: HidReaderWriter<'a, D, 1, 8>,
}

pub struct CommonUsbDriver<D: Driver<'static>> {
    pub(super) hid: HidReaderWriters<'static, D>,
    pub(super) wakeup_signal: &'static RemoteWakeupSignal,
}

impl<D: Driver<'static>> UsbDriver for CommonUsbDriver<D> {
    async fn wait_ready(&mut self) {
        self.hid.keyboard.ready().await;
    }

    async fn send_report(
        &mut self,
        report: rktk::interface::usb::HidReport,
    ) -> Result<(), rktk::interface::error::RktkError> {
        match report {
            rktk::interface::usb::HidReport::Keyboard(report) => {
                if SUSPENDED.load(core::sync::atomic::Ordering::SeqCst) {
                    self.wakeup_signal.signal(());
                    return Ok(());
                }
                let _ = self.hid.keyboard.write_serialize(&report).await;
                Ok(())
            }
            rktk::interface::usb::HidReport::MediaKeyboard(report) => {
                let _ = self.hid.media_key.write_serialize(&report).await;
                Ok(())
            }
            rktk::interface::usb::HidReport::Mouse(report) => {
                let _ = self.hid.mouse.write_serialize(&report).await;
                Ok(())
            }
        }
    }

    async fn wakeup(&mut self) -> Result<(), rktk::interface::error::RktkError> {
        self.wakeup_signal.signal(());
        Ok(())
    }
}
