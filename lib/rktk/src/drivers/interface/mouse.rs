pub trait MouseDriver {
    type Error: core::error::Error;

    async fn init(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn read(&mut self) -> Result<(i8, i8), Self::Error>;
    async fn set_cpi(&mut self, _cpi: u16) -> Result<(), Self::Error>;
    async fn get_cpi(&mut self) -> Result<u16, Self::Error>;
}
