use embassy_nrf::buffered_uarte::BufferedUarte;
use embassy_nrf::timer::Instance as TimerInstance;
use embassy_nrf::uarte::Instance as UarteInstance;
use embedded_io_async::Write as _;
use rktk::drivers::interface::split::SplitDriver;

#[derive(Debug, thiserror::Error)]
pub enum UartFullDuplexSplitDriverError {
    #[error("General error: {0}")]
    GeneralError(&'static str),
}

pub struct UartFullDuplexSplitDriver<I: UarteInstance + 'static, T: TimerInstance + 'static> {
    uarte: BufferedUarte<'static, I, T>,
}

impl<I: UarteInstance + 'static, T: TimerInstance + 'static> UartFullDuplexSplitDriver<I, T> {
    pub fn new(uarte: BufferedUarte<'static, I, T>) -> Self {
        Self { uarte }
    }
}

impl<I: UarteInstance + 'static, T: TimerInstance + 'static> SplitDriver
    for UartFullDuplexSplitDriver<I, T>
{
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
