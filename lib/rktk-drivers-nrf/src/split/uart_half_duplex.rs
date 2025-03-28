//! Uart half duplex split driver
//!
//! This module includes a half-duplex communication driver for split keyboard-to-keyboard communication using UART.
//! Since this driver is a half-duplex communication where the transmitter and receiver share pins, a TRRS cable is not required for connection; a TRS cable is sufficient.
//! However, due to its nature, it is relatively prone to transmission and reception errors. I checked on the receiving side and confirmed that reception failed at a rate of about 0.3%. This is a relatively high figure for a keyboard.

use embassy_nrf::{
    Peripheral,
    buffered_uarte::{BufferedUarteRx, BufferedUarteTx, InterruptHandler},
    gpio::{AnyPin, Flex},
    interrupt,
    ppi::AnyGroup,
    uarte::{Baudrate, Instance, Parity},
};
use embedded_io_async::{Read as _, Write};
use rktk::drivers::interface::split::SplitDriver;

#[derive(Debug, thiserror::Error)]
pub enum UartHalfDuplexSplitDriverError {
    #[error("General error: {0}")]
    GeneralError(&'static str),
}

pub struct UartHalfDuplexSplitDriver<
    UARTE: Instance,
    UARTEP: Peripheral<P = UARTE>,
    IRQ: interrupt::typelevel::Binding<UARTE::Interrupt, InterruptHandler<UARTE>>,
    TIMER: embassy_nrf::timer::Instance,
    TIMERP: Peripheral<P = TIMER>,
    CH1: embassy_nrf::ppi::ConfigurableChannel,
    CH1P: Peripheral<P = CH1>,
    CH2: embassy_nrf::ppi::ConfigurableChannel,
    CH2P: Peripheral<P = CH2>,
> {
    pin: AnyPin,
    uarte: UARTEP,
    irq: IRQ,
    timer: TIMERP,
    ppi_ch1: CH1P,
    ppi_ch2: CH2P,
    ppi_group: AnyGroup,
    read_buffer: [u8; 256],
    write_buffer: [u8; 256],
}

impl<
    UARTE: Instance,
    UARTEP: Peripheral<P = UARTE>,
    IRQ: interrupt::typelevel::Binding<UARTE::Interrupt, InterruptHandler<UARTE>> + Clone,
    TIMER: embassy_nrf::timer::Instance,
    TIMERP: Peripheral<P = TIMER>,
    CH1: embassy_nrf::ppi::ConfigurableChannel,
    CH1P: Peripheral<P = CH1>,
    CH2: embassy_nrf::ppi::ConfigurableChannel,
    CH2P: Peripheral<P = CH2>,
> UartHalfDuplexSplitDriver<UARTE, UARTEP, IRQ, TIMER, TIMERP, CH1, CH1P, CH2, CH2P>
{
    pub fn new(
        mut pin: AnyPin,
        uarte: UARTEP,
        irq: IRQ,
        timer: TIMERP,
        ppi_ch1: CH1P,
        ppi_ch2: CH2P,
        ppi_group: AnyGroup,
    ) -> Self {
        {
            let mut pin = Flex::new(&mut pin);
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
    UARTE: Instance,
    UARTEP: Peripheral<P = UARTE> + 'static,
    IRQ: interrupt::typelevel::Binding<UARTE::Interrupt, InterruptHandler<UARTE>> + Clone + 'static,
    TIMER: embassy_nrf::timer::Instance,
    TIMERP: Peripheral<P = TIMER> + 'static,
    CH1: embassy_nrf::ppi::ConfigurableChannel,
    CH1P: Peripheral<P = CH1> + 'static,
    CH2: embassy_nrf::ppi::ConfigurableChannel,
    CH2P: Peripheral<P = CH2> + 'static,
> SplitDriver
    for UartHalfDuplexSplitDriver<UARTE, UARTEP, IRQ, TIMER, TIMERP, CH1, CH1P, CH2, CH2P>
{
    type Error = UartHalfDuplexSplitDriverError;

    async fn recv(&mut self, buf: &mut [u8], _is_master: bool) -> Result<usize, Self::Error> {
        let mut config = embassy_nrf::uarte::Config::default();
        config.baudrate = Baudrate::BAUD1M;
        config.parity = Parity::EXCLUDED;
        let mut rx = BufferedUarteRx::new(
            &mut self.uarte,
            &mut self.timer,
            &mut self.ppi_ch1,
            &mut self.ppi_ch2,
            &mut self.ppi_group,
            self.irq.clone(),
            &mut self.pin,
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
            &mut self.uarte,
            self.irq.clone(),
            &mut self.pin,
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
