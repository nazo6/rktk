//! Uart half duplex split driver
//!
//! This module includes a half-duplex communication driver for split keyboard-to-keyboard communication using UART.
//! Since this driver is a half-duplex communication where the transmitter and receiver share pins, a TRRS cable is not required for connection; a TRS cable is sufficient.
//! However, due to its nature, it is relatively prone to transmission and reception errors. I checked on the receiving side and confirmed that reception failed at a rate of about 0.3%. This is a relatively high figure for a keyboard.

use embassy_nrf::{
    Peri,
    buffered_uarte::{BufferedUarteRx, BufferedUarteTx, InterruptHandler},
    gpio::{Flex, Pin},
    interrupt,
    uarte::{Baudrate, Instance, Parity},
};
use embedded_io_async::{Read as _, Write};
use rktk::drivers::interface::split::SplitDriver;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartHalfDuplexSplitDriverError {
    GeneralError(&'static str),
}

impl rktk::drivers::interface::Error for UartHalfDuplexSplitDriverError {}

pub struct UartHalfDuplexSplitDriver<
    PIN: Pin,
    UARTE: Instance,
    IRQ: interrupt::typelevel::Binding<UARTE::Interrupt, InterruptHandler<UARTE>>,
    TIMER: embassy_nrf::timer::Instance,
    CH1: embassy_nrf::ppi::ConfigurableChannel,
    CH2: embassy_nrf::ppi::ConfigurableChannel,
    GROUP: embassy_nrf::ppi::Group,
> {
    pin: Peri<'static, PIN>,
    uarte: Peri<'static, UARTE>,
    irq: IRQ,
    timer: Peri<'static, TIMER>,
    ppi_ch1: Peri<'static, CH1>,
    ppi_ch2: Peri<'static, CH2>,
    ppi_group: Peri<'static, GROUP>,
    read_buffer: [u8; 256],
    write_buffer: [u8; 256],
}

impl<
    PIN: Pin,
    UARTE: Instance,
    IRQ: interrupt::typelevel::Binding<UARTE::Interrupt, InterruptHandler<UARTE>> + Clone,
    TIMER: embassy_nrf::timer::Instance,
    CH1: embassy_nrf::ppi::ConfigurableChannel,
    CH2: embassy_nrf::ppi::ConfigurableChannel,
    GROUP: embassy_nrf::ppi::Group,
> UartHalfDuplexSplitDriver<PIN, UARTE, IRQ, TIMER, CH1, CH2, GROUP>
{
    pub fn new(
        mut pin: Peri<'static, PIN>,
        uarte: Peri<'static, UARTE>,
        irq: IRQ,
        timer: Peri<'static, TIMER>,
        ppi_ch1: Peri<'static, CH1>,
        ppi_ch2: Peri<'static, CH2>,
        ppi_group: Peri<'static, GROUP>,
    ) -> Self {
        {
            let mut pin = Flex::new(pin.reborrow());
            pin.set_as_input_output(
                embassy_nrf::gpio::Pull::Up,
                embassy_nrf::gpio::OutputDrive::HighDrive0Disconnect1,
            );
        }
        Self {
            pin,
            uarte,
            irq,
            timer,
            ppi_ch1,
            ppi_ch2,
            ppi_group,
            read_buffer: [0; 256],
            write_buffer: [0; 256],
        }
    }
}

impl<
    PIN: Pin,
    UARTE: Instance,
    IRQ: interrupt::typelevel::Binding<UARTE::Interrupt, InterruptHandler<UARTE>> + Clone + 'static,
    TIMER: embassy_nrf::timer::Instance,
    CH1: embassy_nrf::ppi::ConfigurableChannel,
    CH2: embassy_nrf::ppi::ConfigurableChannel,
    GROUP: embassy_nrf::ppi::Group,
> SplitDriver for UartHalfDuplexSplitDriver<PIN, UARTE, IRQ, TIMER, CH1, CH2, GROUP>
{
    type Error = UartHalfDuplexSplitDriverError;

    async fn recv(&mut self, buf: &mut [u8], _is_master: bool) -> Result<usize, Self::Error> {
        let mut config = embassy_nrf::uarte::Config::default();
        config.baudrate = Baudrate::BAUD1M;
        config.parity = Parity::EXCLUDED;
        let mut rx = BufferedUarteRx::new(
            self.uarte.reborrow(),
            self.timer.reborrow(),
            self.ppi_ch1.reborrow(),
            self.ppi_ch2.reborrow(),
            self.ppi_group.reborrow(),
            self.irq.clone(),
            self.pin.reborrow(),
            config,
            &mut self.read_buffer,
        );
        let mut reader = [0u8];
        let mut i = 0;
        loop {
            rx.read_exact(&mut reader)
                .await
                .map_err(|_| UartHalfDuplexSplitDriverError::GeneralError("read error"))?;
            if reader[0] == 0 {
                buf[i] = reader[0];
                break;
            } else {
                buf[i] = reader[0];
                i += 1;
            }
        }
        drop(rx);

        Ok(i)
    }

    async fn send_all(&mut self, buf: &[u8], _is_master: bool) -> Result<(), Self::Error> {
        let mut config = embassy_nrf::uarte::Config::default();
        config.baudrate = Baudrate::BAUD1M;
        config.parity = Parity::EXCLUDED;
        let mut tx = BufferedUarteTx::new(
            self.uarte.reborrow(),
            self.pin.reborrow(),
            self.irq.clone(),
            config,
            &mut self.write_buffer,
        );

        tx.write_all(buf)
            .await
            .map_err(|_| UartHalfDuplexSplitDriverError::GeneralError("write error"))?;
        tx.flush()
            .await
            .map_err(|_| UartHalfDuplexSplitDriverError::GeneralError("flush error"))?;
        drop(tx);

        embassy_time::Timer::after_micros(50).await;

        Ok(())
    }
}
