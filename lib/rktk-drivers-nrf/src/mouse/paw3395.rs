use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive, Pin},
    spim::{self, Instance, Spim},
    Peripheral,
};
use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex};
use rktk_drivers_common::mouse::paw3395::{config::Config, Paw3395Builder};

pub fn create_paw3395<'d, M: RawMutex, T: Instance + 'd, CS: Peripheral<P = impl Pin> + 'd>(
    shared_spi: &'d Mutex<M, Spim<'d, T>>,
    ncs: CS,
    config: Config,
) -> Paw3395Builder<SpiDevice<'d, M, Spim<'d, T>, Output<'d>>> {
    let ncs = Output::new(ncs, Level::High, OutputDrive::Standard);

    let device = SpiDevice::new(shared_spi, ncs);

    Paw3395Builder::new(device, config)
}

pub fn recommended_paw3395_config() -> spim::Config {
    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M8;
    config.mode.polarity = spim::Polarity::IdleHigh;
    config.mode.phase = spim::Phase::CaptureOnSecondTransition;
    config
}
