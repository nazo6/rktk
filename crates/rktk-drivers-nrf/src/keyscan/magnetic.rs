use embassy_nrf::saadc::{self, Saadc};
use rktk::drivers::interface::magnetic::Adc;

pub struct NrfAdc<'a, const N: usize> {
    saadc: Saadc<'a, N>,
}

impl<'a, const N: usize> NrfAdc<'a, N> {
    pub fn new(saadc: Saadc<'a, N>) -> Self {
        Self { saadc }
    }
}

impl<'a, const N: usize> Adc for NrfAdc<'a, N> {
    type Error = saadc::Error;

    async fn read(&mut self) -> Result<u16, Self::Error> {
        let mut buf = [0; 1];
        self.saadc.sample(&mut buf).await;
        // nRF SAADC returns i16, but for magnetic switches we usually want positive values.
        // Scale/convert appropriately if needed. Here we just take the raw value as u16.
        Ok(buf[0] as u16)
    }
}
