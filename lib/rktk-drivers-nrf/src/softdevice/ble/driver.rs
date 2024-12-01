use rktk::drivers::interface::{ble::BleDriver, reporter::ReporterDriver};
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::{bonder::BOND_FLASH, HidReport, REPORT_CHAN};

pub struct NrfBleDriver {}

#[derive(Debug, thiserror::Error)]
pub enum BleError {
    #[error("Channel full: {0}")]
    ReportChannelFull(&'static str),
    #[error("Not supported")]
    NotSupported,
}

impl ReporterDriver for NrfBleDriver {
    type Error = BleError;

    fn try_send_keyboard_report(&self, report: KeyboardReport) -> Result<(), Self::Error> {
        REPORT_CHAN
            .try_send(HidReport::Keyboard(report))
            .map_err(|_| BleError::ReportChannelFull("keyboard"))?;
        Ok(())
    }

    fn try_send_media_keyboard_report(
        &self,
        report: MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        REPORT_CHAN
            .try_send(HidReport::MediaKeyboard(report))
            .map_err(|_| BleError::ReportChannelFull("media keyboard"))?;

        Ok(())
    }

    fn try_send_mouse_report(&self, report: MouseReport) -> Result<(), Self::Error> {
        REPORT_CHAN
            .try_send(HidReport::Mouse(report))
            .map_err(|_| BleError::ReportChannelFull("mouse"))?;

        Ok(())
    }

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        Err(BleError::NotSupported)
    }

    fn wakeup(&self) -> Result<(), Self::Error> {
        Err(BleError::NotSupported)
    }
}
impl BleDriver for NrfBleDriver {
    type Error = BleError;

    async fn clear_bond_data(&self) -> Result<(), <Self as BleDriver>::Error> {
        BOND_FLASH.signal(super::bonder::BondFlashCommand::Clear);
        Ok(())
    }
}
