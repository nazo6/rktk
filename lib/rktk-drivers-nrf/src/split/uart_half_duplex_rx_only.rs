//! Rx-only driver for half-duplex UART communication.
//! This is for testing purposes only.

use embassy_nrf::{
    buffered_uarte::{BufferedUarteRx, InterruptHandler},
    gpio::{AnyPin, Flex},
    interrupt,
    ppi::{AnyGroup, ConfigurableChannel},
    uarte::Instance,
    Peripheral,
};
use embedded_io_async::Read as _;
use rktk::drivers::interface::split::SplitDriver;

pub struct UartHalfDuplexSplitDriverRx<'a, UARTE: Instance, TIMER: embassy_nrf::timer::Instance> {
    rx: BufferedUarteRx<'a, UARTE, TIMER>,
}

impl<'a, UARTE: Instance, TIMER: embassy_nrf::timer::Instance>
    UartHalfDuplexSplitDriverRx<'a, UARTE, TIMER>
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mut pin: AnyPin,
        uarte: impl Peripheral<P = UARTE> + 'a,
        irq: impl interrupt::typelevel::Binding<UARTE::Interrupt, InterruptHandler<UARTE>> + 'a,
        timer: impl Peripheral<P = TIMER> + 'a,
        ppi_ch1: impl Peripheral<P = impl ConfigurableChannel> + 'a,
        ppi_ch2: impl Peripheral<P = impl ConfigurableChannel> + 'a,
        ppi_group: AnyGroup,
        buffer: &'a mut [u8],
    ) -> Self {
        {
            let mut pin = Flex::new(&mut pin);
            pin.set_as_input_output(
                embassy_nrf::gpio::Pull::Up,
                embassy_nrf::gpio::OutputDrive::HighDrive0Disconnect1,
            );
        }

        let mut config = embassy_nrf::uarte::Config::default();
        config.baudrate = embassy_nrf::uarte::Baudrate::BAUD1M;
        config.parity = embassy_nrf::uarte::Parity::EXCLUDED;

        let rx = BufferedUarteRx::new(
            uarte, timer, ppi_ch1, ppi_ch2, ppi_group, irq, pin, config, buffer,
        );

        Self { rx }
    }
}

impl<'a, UARTE: Instance, TIMER: embassy_nrf::timer::Instance> SplitDriver
    for UartHalfDuplexSplitDriverRx<'a, UARTE, TIMER>
{
    async fn wait_recv(
        &mut self,
        buf: &mut [u8],
        _is_master: bool,
    ) -> Result<(), rktk::drivers::interface::error::RktkError> {
        let mut reader = [0u8];
        let mut i = 0;
        loop {
            self.rx
                .read_exact(&mut reader)
                .await
                .map_err(|_| rktk::drivers::interface::error::RktkError::GeneralError("read error"))?;
            if reader[0] == 0 {
                buf[i] = reader[0];
                break;
            } else {
                buf[i] = reader[0];
                i += 1;
            }
        }

        Ok(())
    }

    async fn send(
        &mut self,
        _buf: &[u8],
        _is_master: bool,
    ) -> Result<(), rktk::drivers::interface::error::RktkError> {
        Ok(())
    }
}
