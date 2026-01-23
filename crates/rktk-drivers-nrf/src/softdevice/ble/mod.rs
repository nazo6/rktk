use nrf_softdevice::{Softdevice, raw};

use rktk::{
    drivers::interface::wireless::WirelessReporterDriverBuilder,
    utils::{Channel, Signal},
};
pub use server::Server;
pub use services::device_information::DeviceInformation;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::softdevice::flash::SoftdeviceFlashPartition;

mod bonder;
mod constant;
mod driver;
mod server;
mod services;
mod task;

#[derive(Debug)]
pub enum InputReport {
    Keyboard(KeyboardReport),
    MediaKeyboard(MediaKeyboardReport),
    Mouse(MouseReport),
}

// Channel for input (device to host) report
static INPUT_REPORT_CHAN: Channel<InputReport, 8> = Channel::new();
// Channel for keyboard output report (only leds field)
static KB_OUTPUT_LED_SIGNAL: Signal<u8> = Signal::new();

pub fn init_ble_server(sd: &mut Softdevice, device_info: DeviceInformation) -> Server {
    unsafe {
        raw::sd_ble_gap_appearance_set(raw::BLE_APPEARANCE_HID_KEYBOARD as u16);
    }

    server::Server::new(sd, device_info).unwrap()
}

pub struct SoftdeviceBleReporterBuilder {
    sd: &'static Softdevice,
    server: Server,
    name: &'static str,
    flash: SoftdeviceFlashPartition,
    spawner: embassy_executor::Spawner,
}

impl SoftdeviceBleReporterBuilder {
    pub fn new(
        spawner: embassy_executor::Spawner,
        sd: &'static Softdevice,
        server: Server,
        name: &'static str,
        flash: SoftdeviceFlashPartition,
    ) -> Self {
        Self {
            sd,
            server,
            name,
            flash,
            spawner,
        }
    }
}

impl WirelessReporterDriverBuilder for SoftdeviceBleReporterBuilder {
    type Output = driver::SoftdeviceBleReporterDriver;

    type Error = ();

    async fn build(self) -> Result<(Self::Output, impl Future<Output = ()>), Self::Error> {
        Ok((
            driver::SoftdeviceBleReporterDriver {},
            task::softdevice_task(self.spawner, self.sd, self.server, self.name, self.flash),
        ))
    }
}
