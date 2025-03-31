use nrf_softdevice::{Softdevice, raw};

use rktk::{drivers::interface::ble::BleDriverBuilder, utils::Channel};
pub use server::Server;
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

static REPORT_CHAN: Channel<HidReport, 8> = Channel::new();

pub fn init_ble_server(sd: &mut Softdevice, device_info: DeviceInformation) -> Server {
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
    pub fn new(
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

impl BleDriverBuilder for NrfBleDriverBuilder {
    type Output = driver::NrfBleDriver;

    type Error = ();

    async fn build(self) -> Result<(Self::Output, impl Future<Output = ()>), Self::Error> {
        Ok((
            driver::NrfBleDriver {},
            task::softdevice_task(self.sd, self.server, self.name, self.flash),
        ))
    }
}
