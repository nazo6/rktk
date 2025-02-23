use std::sync::Arc;

use async_hid::{Device, DeviceInfo};
use futures::stream::StreamExt;
use rktk_rrp::transport::{ReadTransport, WriteTransport};

use super::{RrpHidBackend, RrpHidDevice};

pub struct NativeBackend {}

impl RrpHidBackend for NativeBackend {
    type Error = anyhow::Error;

    type HidDevice = NativeHidDevice;

    async fn open_device(
        &self,
        usage_page: u16,
        usage: u16,
    ) -> Result<Self::HidDevice, Self::Error> {
        let mut device = None;
        let mut devices = DeviceInfo::enumerate().await?;
        while let Some(info) = devices.next().await {
            if info.usage_page == usage_page && info.usage_id == usage {
                device = Some(info);
            }
        }

        let Some(device) = device else {
            return Err(anyhow::anyhow!("Device not found"));
        };

        let device = device.open(async_hid::AccessMode::ReadWrite).await?;
        let device = Arc::new(device);

        Ok(NativeHidDevice {
            client: rktk_rrp::client::Client::new(
                HidReader {
                    device: device.clone(),
                    remained: Vec::new(),
                },
                HidWriter {
                    device: device.clone(),
                },
            ),
        })
    }

    fn set_ondisconnect(&self, fun: Option<impl FnMut() + 'static>) {}

    fn new() -> Self {
        Self {}
    }
}

pub struct NativeHidDevice {
    client: rktk_rrp::client::Client<HidReader, HidWriter>,
}
impl RrpHidDevice for NativeHidDevice {
    type Error = anyhow::Error;

    type ReadTransport = HidReader;

    type WriteTransport = HidWriter;

    async fn close(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_client(
        &mut self,
    ) -> &mut rktk_rrp::client::Client<Self::ReadTransport, Self::WriteTransport> {
        &mut self.client
    }
}

pub struct HidReader {
    device: Arc<Device>,
    remained: Vec<u8>,
}

impl ReadTransport for HidReader {
    type Error = anyhow::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        while self.remained.len() < buf.len() {
            // One hid report is consist of
            // data length: 1byte
            // data:        31byte
            let mut tmp_buf = [0; 32];
            let _ = self.device.read_input_report(&mut tmp_buf).await?;
            let size = tmp_buf[0] as usize;
            let read_data = &tmp_buf[1..=size];
            self.remained.extend_from_slice(read_data);
        }

        buf.copy_from_slice(&self.remained[..buf.len()]);
        self.remained.drain(..buf.len());

        Ok(buf.len())
    }
}

pub struct HidWriter {
    device: Arc<Device>,
}

impl WriteTransport for HidWriter {
    type Error = anyhow::Error;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for chunk in buf.chunks(31) {
            // When sending, first byte is report id.
            let mut data = vec![0, chunk.len() as u8];
            data.extend_from_slice(chunk);
            data.resize(33, 0);
            self.device.write_output_report(&data).await?;
        }

        Ok(buf.len())
    }
}
