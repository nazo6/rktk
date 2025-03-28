use core::convert::Infallible;

use rktk::{
    drivers::interface::{ble::BleDriver, reporter::ReporterDriver},
    utils::Sender,
};
use usbd_hid::descriptor::KeyboardReport;

pub struct TroubleReporter {
    pub(super) output_tx: Sender<'static, KeyboardReport, 4>,
}

impl ReporterDriver for TroubleReporter {
    type Error = Infallible;

    fn try_send_keyboard_report(
        &self,
        report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        self.output_tx.try_send(report);
        Ok(())
    }

    fn try_send_media_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn try_send_mouse_report(
        &self,
        _report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn wakeup(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}
impl BleDriver for TroubleReporter {
    type Error = Infallible;

    async fn clear_bond_data(&self) -> Result<(), <Self as BleDriver>::Error> {
        Ok(())
    }
}
