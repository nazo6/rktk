use super::error::RktkError;

pub trait Mouse {
    async fn init(&mut self) -> Result<(), RktkError> {
        Ok(())
    }
    async fn read(&mut self) -> Result<(i8, i8), RktkError>;
    async fn set_cpi(&mut self, cpi: u16) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
    async fn get_cpi(&mut self) -> Result<u16, RktkError> {
        Err(RktkError::NotSupported)
    }
}

pub struct DummyMouse;
impl Mouse for DummyMouse {
    async fn read(&mut self) -> Result<(i8, i8), RktkError> {
        Err(RktkError::NotSupported)
    }
}
