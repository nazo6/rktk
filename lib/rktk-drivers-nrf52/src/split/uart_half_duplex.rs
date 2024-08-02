use embassy_nrf::{
    buffered_uarte::{BufferedUarteRx, BufferedUarteTx, InterruptHandler},
    gpio::{AnyPin, Flex},
    interrupt,
    ppi::AnyGroup,
    uarte::Instance,
    Peripheral,
};
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
    read_buffer: [u8; 8],
    write_buffer: [u8; 8],
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
            read_buffer: [0; 8],
            write_buffer: [0; 8],
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
    async fn wait_recv(&mut self, buf: &mut [u8]) -> Result<(), rktk::interface::error::RktkError> {
        // {
        //     let mut flex = Flex::new(&mut self.pin);
        //     flex.set_as_input(embassy_nrf::gpio::Pull::Up);
        // }
        let config = embassy_nrf::uarte::Config::default();
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
        rx.read(buf).await.map_err(|e| {
            rktk::print!("{:?} {}", e, embassy_time::Instant::now());
            rktk::interface::error::RktkError::GeneralError("read error")
        })?;
        Ok(())
    }

    async fn send(&mut self, buf: &[u8]) -> Result<(), rktk::interface::error::RktkError> {
        // {
        //     let mut flex = Flex::new(&mut self.pin);
        //     flex.set_as_input_output(
        //         embassy_nrf::gpio::Pull::None,
        //         embassy_nrf::gpio::OutputDrive::Standard0Disconnect1,
        //     );
        // }
        let config = embassy_nrf::uarte::Config::default();
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
        Ok(())
    }
}
