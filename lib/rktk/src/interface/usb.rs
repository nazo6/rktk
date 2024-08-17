use super::{error::RktkError, reporter::ReporterDriver, DriverBuilder};

pub trait UsbDriver: ReporterDriver {
    async fn vbus_detect(&self) -> Result<bool, RktkError> {
        Err(RktkError::NotSupported)
    }
}

pub enum DummyUsbDriver {}
impl ReporterDriver for DummyUsbDriver {}
impl UsbDriver for DummyUsbDriver {}

pub enum DummyUsbDriverBuilder {}
impl DriverBuilder for DummyUsbDriverBuilder {
    type Output = DummyUsbDriver;

    type Error = ();

    async fn build(self) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
