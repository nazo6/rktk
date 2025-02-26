use embassy_nrf::radio::Instance;
use embassy_nrf_esb::prx::PrxRadio;
use rktk::drivers::interface::dongle::{DongleData, DongleDriver};

pub struct EsbDongleDrier<T: Instance> {
    prx: PrxRadio<'static, T, 64>,
}

impl<T: Instance> EsbDongleDrier<T> {
    pub fn new(prx: PrxRadio<'static, T, 64>) -> Self {
        Self { prx }
    }
}

impl<T: Instance> DongleDriver for EsbDongleDrier<T> {
    type Error = ();

    async fn recv(&mut self) -> Result<DongleData, Self::Error> {
        let mut buf = [0; 64];
        let size = self.prx.recv(&mut buf, 0xFF).await.map_err(|_| ())?;
        let data = postcard::from_bytes(&buf[..size]).map_err(|_| ())?;

        Ok(data)
    }
}
