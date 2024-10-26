use embassy_rp::{
    dma::Channel,
    gpio::{Output, Pin},
    spi::{Async, ClkPin, Instance, MisoPin, MosiPin, Spi},
    Peripheral,
};
pub use rktk_drivers_common::mouse::paw3395::config;
use rktk_drivers_common::mouse::paw3395::Paw3395Builder;

pub fn create_paw3395<'d, T: Instance + 'd>(
    inner: impl Peripheral<P = T> + 'd,
    clk: impl Peripheral<P = impl ClkPin<T> + 'd> + 'd,
    mosi: impl Peripheral<P = impl MosiPin<T> + 'd> + 'd,
    miso: impl Peripheral<P = impl MisoPin<T> + 'd> + 'd,
    tx_dma: impl Peripheral<P = impl Channel> + 'd,
    rx_dma: impl Peripheral<P = impl Channel> + 'd,
    ncs: impl Peripheral<P = impl Pin> + 'd,
    config: config::Config,
) -> Paw3395Builder<'static, Spi<'d, T, Async>, Output<'d>> {
    let mut spi_config = embassy_rp::spi::Config::default();
    spi_config.frequency = 7_000_000;
    spi_config.polarity = embassy_rp::spi::Polarity::IdleHigh;
    spi_config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    let ncs = Output::new(ncs, embassy_rp::gpio::Level::High);

    let spi = Spi::new(inner, clk, mosi, miso, tx_dma, rx_dma, spi_config);
    Paw3395Builder::new(spi, ncs, config)
}
