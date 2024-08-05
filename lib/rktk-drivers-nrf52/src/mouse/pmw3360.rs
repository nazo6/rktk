use embassy_nrf::{
    gpio::{Level, Output, OutputDrive, Pin},
    interrupt::typelevel::Binding,
    spim::{Config, Frequency, Instance, InterruptHandler, Spim},
    spis::{Phase, Polarity},
    Peripheral,
};
use rktk_drivers_common::mouse::pmw3360::Pmw3360Builder;

pub fn create_pmw3360<'d, T: Instance + 'd>(
    spim: impl Peripheral<P = T> + 'd,
    _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    sck: impl Peripheral<P = impl Pin + 'd> + 'd,
    miso: impl Peripheral<P = impl Pin + 'd> + 'd,
    mosi: impl Peripheral<P = impl Pin + 'd> + 'd,
    ncs: impl Peripheral<P = impl Pin> + 'd,
) -> Pmw3360Builder<'d, Spim<'d, T>, Output<'d>> {
    let mut config = Config::default();
    config.frequency = Frequency::M8;
    config.mode.polarity = Polarity::IdleHigh;
    config.mode.phase = Phase::CaptureOnSecondTransition;
    let ncs = Output::new(ncs, Level::High, OutputDrive::Standard);

    let spi = Spim::new(spim, _irq, sck, miso, mosi, config);
    Pmw3360Builder::new(spi, ncs)
}
