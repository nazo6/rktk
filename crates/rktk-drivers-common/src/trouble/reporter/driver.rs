use core::convert::Infallible;

use rktk::{
    drivers::interface::{reporter::ReporterDriver, wireless::WirelessReporterDriver},
    utils::Sender,
};

use super::Report;

pub struct TroubleReporter {
    pub(super) output_tx: Sender<'static, Report, 4>,
}

impl ReporterDriver for TroubleReporter {
    type Error = Infallible;

    fn try_send_keyboard_report(
        &self,
        report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        let _ = self.output_tx.try_send(Report::Keyboard(report));
        Ok(())
    }

    fn try_send_media_keyboard_report(
        &self,
        report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        let _ = self.output_tx.try_send(Report::MediaKeyboard(report));
        Ok(())
    }

    fn try_send_mouse_report(
        &self,
        report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        let _ = self.output_tx.try_send(Report::Mouse(report));
        Ok(())
    }

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn wakeup(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}
impl WirelessReporterDriver for TroubleReporter {
    type Error = Infallible;

    async fn clear_bond_data(&self) -> Result<(), <Self as WirelessReporterDriver>::Error> {
        Ok(())
    }
}
