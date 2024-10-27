use super::{reporter::ReporterDriver, BackgroundTask, DriverBuilderWithTask};

pub trait BleDriver: ReporterDriver {}

pub enum DummyBleDriver {}
impl ReporterDriver for DummyBleDriver {}
impl BleDriver for DummyBleDriver {}

pub enum DummyBleDriverBuilder {}
impl DriverBuilderWithTask for DummyBleDriverBuilder {
    type Driver = DummyBleDriver;

    type Error = ();

    #[allow(refining_impl_trait)]
    async fn build(self) -> Result<(Self::Driver, DummyBleTask), Self::Error> {
        unreachable!()
    }
}

pub enum DummyBleTask {}
impl BackgroundTask for DummyBleTask {
    async fn run(self) {
        unreachable!()
    }
}
