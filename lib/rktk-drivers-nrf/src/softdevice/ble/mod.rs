use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use nrf_softdevice::{raw, Softdevice};

use rktk::drivers::interface::DriverBuilderWithTask;
use server::Server;
pub use services::device_information::DeviceInformation;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::flash::SharedFlash;

mod bonder;
mod constant;
mod driver;
mod server;
mod services;
mod task;

#[derive(Debug)]
pub enum HidReport {
    Keyboard(KeyboardReport),
    MediaKeyboard(MediaKeyboardReport),
    Mouse(MouseReport),
}

static REPORT_CHAN: Channel<CriticalSectionRawMutex, HidReport, 8> = Channel::new();

pub async fn init_ble_server(sd: &mut Softdevice, device_info: DeviceInformation) -> Server {
    unsafe {
        raw::sd_ble_gap_appearance_set(raw::BLE_APPEARANCE_HID_KEYBOARD as u16);
    }

    server::Server::new(sd, device_info).unwrap()
}

pub struct NrfBleDriverBuilder {
    sd: &'static Softdevice,
    server: Server,
    name: &'static str,
    flash: &'static SharedFlash,
}

impl NrfBleDriverBuilder {
    pub async fn new(
        sd: &'static Softdevice,
        server: Server,
        name: &'static str,
        flash: &'static SharedFlash,
    ) -> Self {
        Self {
            sd,
            server,
            name,
            flash,
        }
    }
}

impl DriverBuilderWithTask for NrfBleDriverBuilder {
    type Driver = driver::NrfBleDriver;

    type Error = ();

    async fn build(
        self,
    ) -> Result<(Self::Driver, impl rktk::drivers::interface::BackgroundTask), Self::Error> {
        Ok((
            driver::NrfBleDriver {},
            task::SoftdeviceBleTask {
                sd: self.sd,
                server: self.server,
                name: self.name,
                flash: self.flash,
            },
        ))
    }
}
