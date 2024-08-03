use embassy_nrf::{
    buffered_uarte::{BufferedUarteRx, BufferedUarteTx, InterruptHandler},
    gpio::{AnyPin, Input, Output},
    interrupt,
    ppi::AnyGroup,
    uarte::{Baudrate, Instance, Parity},
    Peripheral,
};
use embedded_io_async::Read as _;
use rktk::interface::split::SplitDriver;

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
    read_buffer: [u8; 64],
    write_buffer: [u8; 64],
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
        pin: AnyPin,
        uarte: UARTEP,
        irq: IRQ,
        timer: TIMERP,
        ppi_ch1: CH1P,
        ppi_ch2: CH2P,
        ppi_group: AnyGroup,
    ) -> Self {
        Self {
            pin,
            uarte,
            irq,
            timer,
            ppi_ch1,
            ppi_ch2,
            ppi_group,
            read_buffer: [0; 64],
            write_buffer: [0; 64],
        }
    }
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
    > SplitDriver
    for UartHalfDuplexSplitDriver<UARTE, UARTEP, IRQ, TIMER, TIMERP, CH1, CH1P, CH2, CH2P>
{
    async fn wait_recv(
        &mut self,
        buf: &mut [u8],
        _is_master: bool,
    ) -> Result<(), rktk::interface::error::RktkError> {
        {
            let _pin = Input::new(&mut self.pin, embassy_nrf::gpio::Pull::Up);
        }
        let mut config = embassy_nrf::uarte::Config::default();
        config.baudrate = Baudrate::BAUD115200;
        config.parity = Parity::INCLUDED;
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
                .map_err(|_| rktk::interface::error::RktkError::GeneralError("read error"))?;
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
        buf: &[u8],
        _is_master: bool,
    ) -> Result<(), rktk::interface::error::RktkError> {
        {
            let _pin = Output::new(
                &mut self.pin,
                embassy_nrf::gpio::Level::High,
                embassy_nrf::gpio::OutputDrive::Standard,
            );
        }
        let mut config = embassy_nrf::uarte::Config::default();
        config.baudrate = Baudrate::BAUD115200;
        config.parity = Parity::INCLUDED;
        let mut tx = BufferedUarteTx::new(
            &mut self.uarte,
            self.irq.clone(),
            &mut self.pin,
            config,
            &mut self.write_buffer,
        );
        tx.write(buf)
            .await
            .map_err(|_| rktk::interface::error::RktkError::GeneralError("write error"))?;
        tx.flush()
            .await
            .map_err(|_| rktk::interface::error::RktkError::GeneralError("flush error"))?;
        Ok(())
    }
}
