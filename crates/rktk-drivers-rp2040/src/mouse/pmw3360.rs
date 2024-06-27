use embassy_rp::{
    dma::Channel,
    gpio::{Output, Pin},
    spi::{Async, ClkPin, Instance, MisoPin, MosiPin, Spi},
    Peripheral,
};
use rktk::interface::mouse::Mouse;
use rktk_drivers_common::mouse::pmw3360::{Pmw3360, Pmw3360Error};

pub async fn create_pmw3360<'d, T: Instance + 'd>(
    inner: impl Peripheral<P = T> + 'd,
    clk: impl Peripheral<P = impl ClkPin<T> + 'd> + 'd,
    mosi: impl Peripheral<P = impl MosiPin<T> + 'd> + 'd,
    miso: impl Peripheral<P = impl MisoPin<T> + 'd> + 'd,
    tx_dma: impl Peripheral<P = impl Channel> + 'd,
    rx_dma: impl Peripheral<P = impl Channel> + 'd,
    ncs: impl Peripheral<P = impl Pin> + 'd,
) -> Result<impl Mouse + 'd, Pmw3360Error<Spi<'d, T, Async>>> {
    let mut config = embassy_rp::spi::Config::default();
    config.frequency = 2_000_000;
    config.polarity = embassy_rp::spi::Polarity::IdleHigh;
    config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    let ncs = Output::new(ncs, embassy_rp::gpio::Level::High);

    let spi = Spi::new(inner, clk, mosi, miso, tx_dma, rx_dma, config);
    Pmw3360::new(spi, ncs).await
}
