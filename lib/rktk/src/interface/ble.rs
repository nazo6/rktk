use super::reporter::ReporterDriver;

pub trait BleDriver: ReporterDriver {}

pub enum DummyBleDriver {}
impl ReporterDriver for DummyBleDriver {}
impl BleDriver for DummyBleDriver {}
