use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive, Pin},
    spim::{self, Instance, Spim},
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
    shared_spi: &'a Mutex<M, Spim<'d, T>>,
    ncs: CS,
) -> Pmw3360Builder<SpiDevice<'a, M, Spim<'d, T>, Output<'d>>> {
    let ncs = Output::new(ncs, Level::High, OutputDrive::Standard);

    let device = SpiDevice::new(shared_spi, ncs);

    Pmw3360Builder::new(device)
}

pub fn recommended_pmw3360_config() -> spim::Config {
    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M8;
    config.mode.polarity = spim::Polarity::IdleHigh;
    config.mode.phase = spim::Phase::CaptureOnSecondTransition;
    config
}
