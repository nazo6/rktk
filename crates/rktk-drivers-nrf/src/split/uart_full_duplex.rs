use embassy_nrf::buffered_uarte::BufferedUarte;
use embedded_io_async::Write as _;
use rktk::drivers::interface::split::SplitDriver;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartFullDuplexSplitDriverError {
    GeneralError(&'static str),
}

impl rktk::drivers::interface::Error for UartFullDuplexSplitDriverError {}

pub struct UartFullDuplexSplitDriver {
    uarte: BufferedUarte<'static>,
}

impl UartFullDuplexSplitDriver {
    pub fn new(uarte: BufferedUarte<'static>) -> Self {
        Self { uarte }
    }
}

impl SplitDriver for UartFullDuplexSplitDriver {
    type Error = UartFullDuplexSplitDriverError;

    async fn recv(&mut self, buf: &mut [u8], _is_master: bool) -> Result<usize, Self::Error> {
        let size = self
            .uarte
            .read(buf)
            .await
            .map_err(|_| UartFullDuplexSplitDriverError::GeneralError("Read error"))?;
        Ok(size)
    }

    async fn send_all(&mut self, buf: &[u8], _is_master: bool) -> Result<(), Self::Error> {
        self.uarte
            .write_all(buf)
            .await
            .map_err(|_| UartFullDuplexSplitDriverError::GeneralError("Write error"))
    }
}
