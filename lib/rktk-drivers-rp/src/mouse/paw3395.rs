use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_rp::{
    gpio::{Output, Pin},
    spi::{self, Async, Instance, Spi},
    Peripheral,
};
use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex};
pub use rktk_drivers_common::mouse::paw3395::config;
use rktk_drivers_common::mouse::paw3395::Paw3395Builder;

pub fn create_paw3395<'d, M: RawMutex, T: Instance + 'd, CS: Peripheral<P = impl Pin> + 'd>(
    shared_spi: &'d Mutex<M, Spi<'d, T, Async>>,
    ncs: CS,
    config: config::Config,
) -> Paw3395Builder<SpiDevice<'d, M, Spi<'d, T, Async>, Output<'d>>> {
    // let mut spi_config = embassy_rp::spi::Config::default();
    // spi_config.frequency = 7_000_000;
    // spi_config.polarity = embassy_rp::spi::Polarity::IdleHigh;
    // spi_config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    let ncs = Output::new(ncs, embassy_rp::gpio::Level::High);

    let device = SpiDevice::new(shared_spi, ncs);

    Paw3395Builder::new(device, config)
}

pub fn recommended_spi_config() -> embassy_rp::spi::Config {
    let mut spi_config = spi::Config::default();
    spi_config.frequency = 7_000_000;
    spi_config.polarity = spi::Polarity::IdleHigh;
    spi_config.phase = spi::Phase::CaptureOnSecondTransition;

    spi_config
}
