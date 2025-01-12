use embassy_nrf::buffered_uarte::BufferedUarte;
use embassy_nrf::timer::Instance as TimerInstance;
use embassy_nrf::uarte::Instance as UarteInstance;
use embedded_io_async::{Read, Write as _};
use rktk::drivers::interface::split::SplitDriver;

#[derive(Debug, thiserror::Error)]
pub enum UartFullDuplexSplitDriverError {
    #[error("General error: {0}")]
    GeneralError(&'static str),
}

pub struct UartFullDuplexSplitDriver<'d, I: UarteInstance, T: TimerInstance> {
    uarte: BufferedUarte<'d, I, T>,
}

impl<'d, I: UarteInstance, T: TimerInstance> UartFullDuplexSplitDriver<'d, I, T> {
    pub fn new(uarte: BufferedUarte<'d, I, T>) -> Self {
        Self { uarte }
    }
}

impl<I: UarteInstance, T: TimerInstance> SplitDriver for UartFullDuplexSplitDriver<'_, I, T> {
    type Error = UartFullDuplexSplitDriverError;

    async fn wait_recv(&mut self, buf: &mut [u8], _is_master: bool) -> Result<(), Self::Error> {
        let mut i = 0;
        loop {
            let mut byte = [0];
            self.uarte
                .read_exact(&mut byte)
                .await
                .map_err(|_| UartFullDuplexSplitDriverError::GeneralError("Read error"))?;
            if byte[0] == 0 {
                break;
            } else {
                buf[i] = byte[0];
                i += 1;
            }
        }
        Ok(())
    }

    async fn send(&mut self, buf: &[u8], _is_master: bool) -> Result<(), Self::Error> {
        self.uarte
            .write_all(buf)
            .await
            .map_err(|_| UartFullDuplexSplitDriverError::GeneralError("Write error"))
    }
}
