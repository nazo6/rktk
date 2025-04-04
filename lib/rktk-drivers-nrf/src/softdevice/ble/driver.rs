use rktk::drivers::interface::{ble::BleDriver, reporter::ReporterDriver};
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::{INPUT_REPORT_CHAN, InputReport, KB_OUTPUT_LED_SIGNAL, bonder::BOND_FLASH};

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
        INPUT_REPORT_CHAN
            .try_send(InputReport::Keyboard(report))
            .map_err(|_| BleError::ReportChannelFull("keyboard"))?;
        Ok(())
    }

    fn try_send_media_keyboard_report(
        &self,
        report: MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        INPUT_REPORT_CHAN
            .try_send(InputReport::MediaKeyboard(report))
            .map_err(|_| BleError::ReportChannelFull("media keyboard"))?;

        Ok(())
    }

    fn try_send_mouse_report(&self, report: MouseReport) -> Result<(), Self::Error> {
        INPUT_REPORT_CHAN
            .try_send(InputReport::Mouse(report))
            .map_err(|_| BleError::ReportChannelFull("mouse"))?;

        Ok(())
    }

    async fn recv_keyboard_report(&self) -> Result<u8, Self::Error> {
        let leds = KB_OUTPUT_LED_SIGNAL.wait().await;
        Ok(leds)
    }

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        Err(BleError::NotSupported)
    }

    fn wakeup(&self) -> Result<bool, Self::Error> {
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
