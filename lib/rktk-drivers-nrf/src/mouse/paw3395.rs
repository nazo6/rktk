use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive, Pin},
    spim::{Instance, Spim},
    Peripheral,
};
use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex};
use rktk_drivers_common::mouse::paw3395::{config::Config, Paw3395Builder};

pub fn create_paw3395<'d, M: RawMutex, T: Instance + 'd, CS: Peripheral<P = impl Pin> + 'd>(
    shared_spi: &'d Mutex<M, Spim<'d, T>>,
    ncs: CS,
    config: Config,
) -> Paw3395Builder<'d, SpiDevice<'d, M, Spim<'d, T>, Output<'d>>> {
    // let mut config = Config::default();
    // config.frequency = Frequency::M8;
    // config.mode.polarity = Polarity::IdleHigh;
    // config.mode.phase = Phase::CaptureOnSecondTransition;
    let ncs = Output::new(ncs, Level::High, OutputDrive::Standard);

    let device = SpiDevice::new(shared_spi, ncs);

    Paw3395Builder::new(device, config)
}
