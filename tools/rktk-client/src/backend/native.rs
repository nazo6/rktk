use rktk_rrp::transport::{ReadTransport, WriteTransport};

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
        todo!()
    }

    fn set_ondisconnect(&mut self, fun: Option<impl FnMut() + 'static>) {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }
}

pub struct NativeHidDevice {}

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
        todo!()
    }
}

pub struct HidReader {}

impl ReadTransport for HidReader {
    type Error = anyhow::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        todo!()
    }
}

pub struct HidWriter {}

impl WriteTransport for HidWriter {
    type Error = anyhow::Error;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        todo!()
    }
}
