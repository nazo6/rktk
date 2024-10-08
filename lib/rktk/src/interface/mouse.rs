use super::{error::RktkError, DriverBuilder};

pub trait MouseDriver {
    async fn read(&mut self) -> Result<(i8, i8), RktkError>;
    async fn set_cpi(&mut self, _cpi: u16) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
    async fn get_cpi(&mut self) -> Result<u16, RktkError> {
        Err(RktkError::NotSupported)
    }
}

/// Dummy driver that is only used to be given as a type argument.
pub enum DummyMouseDriver {}
impl MouseDriver for DummyMouseDriver {
    async fn read(&mut self) -> Result<(i8, i8), RktkError> {
        Err(RktkError::NotSupported)
    }
}

pub enum DummyMouseDriverBuilder {}
impl DriverBuilder for DummyMouseDriverBuilder {
    type Output = DummyMouseDriver;

    type Error = ();

    async fn build(self) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
