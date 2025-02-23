use std::sync::Arc;

use hidapi::{HidApi, HidDevice};
use rktk_rrp::transport::{ReadTransport, WriteTransport};
use smol::lock::Mutex;

use super::{RrpHidBackend, RrpHidDevice};

pub struct NativeBackend {}

impl RrpHidBackend for NativeBackend {
    type Error = anyhow::Error;

    type HidDevice = NativeHidDevice;

    async fn open_device(
        &mut self,
        usage_page: u16,
        usage: u16,
    ) -> Result<Self::HidDevice, Self::Error> {
        let hid = HidApi::new()?;
        let Some(device) = hid
            .device_list()
            .find(|d| d.usage_page() == usage_page && d.usage() == usage)
        else {
            anyhow::bail!("No devices found")
        };
        let device = device.open_device(&hid)?;

        let d = Arc::new(Mutex::new(device));
        Ok(NativeHidDevice {
            client: rktk_rrp::client::Client::new(
                HidReader { device: d.clone() },
                HidWriter { device: d },
            ),
        })
    }

    fn set_ondisconnect(&mut self, fun: Option<impl FnMut() + 'static>) {}

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
    device: Arc<Mutex<HidDevice>>,
}

impl ReadTransport for HidReader {
    type Error = anyhow::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let device = self.device.clone();
        let mut tmp_buf = buf.to_vec();
        let (tmp_buf, size) = smol::unblock(move || {
            let device = device.lock_blocking();
            match device.read(&mut tmp_buf) {
                Ok(size) => Ok((tmp_buf, size)),
                Err(e) => Err(e),
            }
        })
        .await?;
        buf.copy_from_slice(&tmp_buf);

        Ok(size)
    }
}

pub struct HidWriter {
    device: Arc<Mutex<HidDevice>>,
}

impl WriteTransport for HidWriter {
    type Error = anyhow::Error;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let buf_v = buf.to_vec();
        let device = self.device.clone();
        smol::unblock(move || {
            let device = device.lock_blocking();
            for chunk in buf_v.chunks(31) {
                let mut data = vec![chunk.len() as u8];
                data.extend_from_slice(chunk);
                data.resize(32, 0);
                device.write(&data)?;
            }

            Result::<(), anyhow::Error>::Ok(())
        })
        .await?;

        Ok(buf.len())
    }
}
