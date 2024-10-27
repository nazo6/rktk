use rktk::interface::{ble::BleDriver, error::RktkError, reporter::ReporterDriver};
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::{HidReport, REPORT_CHAN};

pub struct NrfBleDriver {}

impl ReporterDriver for NrfBleDriver {
    fn try_send_keyboard_report(&self, report: KeyboardReport) -> Result<(), RktkError> {
        REPORT_CHAN
            .try_send(HidReport::Keyboard(report))
            .map_err(|_| RktkError::GeneralError("report_chan not empty"))?;
        Ok(())
    }

    fn try_send_media_keyboard_report(&self, report: MediaKeyboardReport) -> Result<(), RktkError> {
        REPORT_CHAN
            .try_send(HidReport::MediaKeyboard(report))
            .map_err(|_| RktkError::GeneralError("report_chan not empty"))?;
        Ok(())
    }

    fn try_send_mouse_report(&self, report: MouseReport) -> Result<(), RktkError> {
        REPORT_CHAN
            .try_send(HidReport::Mouse(report))
            .map_err(|_| RktkError::GeneralError("report_chan not empty"))?;
        Ok(())
    }
}
impl BleDriver for NrfBleDriver {}
