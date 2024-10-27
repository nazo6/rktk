use super::{error::RktkError, reporter::ReporterDriver, BackgroundTask, DriverBuilderWithTask};

pub trait UsbDriver: ReporterDriver {
    async fn vbus_detect(&self) -> Result<bool, RktkError> {
        Err(RktkError::NotSupported)
    }
}

pub enum DummyUsbDriver {}
impl ReporterDriver for DummyUsbDriver {}
impl UsbDriver for DummyUsbDriver {}

pub enum DummyUsbDriverBuilder {}
impl DriverBuilderWithTask for DummyUsbDriverBuilder {
    type Driver = DummyUsbDriver;

    type Error = ();

    #[allow(refining_impl_trait)]
    async fn build(self) -> Result<(Self::Driver, DummyUsbTask), Self::Error> {
        unreachable!()
    }
}

pub enum DummyUsbTask {}
impl BackgroundTask for DummyUsbTask {
    async fn run(self) {
        unreachable!()
    }
}
