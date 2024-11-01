use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_rp::{
    gpio::{Output, Pin},
    spi::{Async, Instance, Spi},
    Peripheral,
};
use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex};
use rktk_drivers_common::mouse::pmw3360::Pmw3360Builder;

pub fn create_pmw3360<
    'a,
    'd: 'a,
    M: RawMutex,
    T: Instance + 'd,
    CS: Peripheral<P = impl Pin> + 'd,
>(
    shared_spi: &'a Mutex<M, Spi<'d, T, Async>>,
    ncs: CS,
) -> Pmw3360Builder<SpiDevice<'a, M, Spi<'d, T, Async>, Output<'d>>> {
    // let mut config = embassy_rp::spi::Config::default();
    // config.frequency = 7_000_000;
    // config.polarity = embassy_rp::spi::Polarity::IdleHigh;
    // config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    let ncs = Output::new(ncs, embassy_rp::gpio::Level::High);

    let device = SpiDevice::new(shared_spi, ncs);

    Pmw3360Builder::new(device)
}
