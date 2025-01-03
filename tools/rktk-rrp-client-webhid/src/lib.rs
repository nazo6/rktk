use async_lock::Mutex;
use client::{HidReader, HidWriter};
use web_sys::HidDevice;

mod client;

pub struct Client {
    pub client: Mutex<rktk_rrp::client::Client<HidReader, HidWriter, 1024>>,
}

impl Client {
    pub fn new(device: &HidDevice) -> Self {
        Client {
            client: Mutex::new(rktk_rrp::client::Client::new(
                HidReader::new(device.clone()),
                HidWriter::new(device.clone()),
            )),
        }
    }
}
