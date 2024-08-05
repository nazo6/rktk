use super::{error::RktkError, usb::HidReport};

pub trait BleDriver {
    async fn wait_ready(&mut self);
    async fn send_report(&mut self, report: HidReport) -> Result<(), RktkError>;
}

pub enum DummyBleDriver {}
impl BleDriver for DummyBleDriver {
    async fn wait_ready(&mut self) {}
    async fn send_report(&mut self, _report: HidReport) -> Result<(), RktkError> {
        Ok(())
    }
}
