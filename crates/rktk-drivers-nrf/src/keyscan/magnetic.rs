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
        let mut buf = [0; N];
        // Double-sample: the first sample pre-charges/discharges the internal S&H capacitor to the channel voltage,
        // and the second sample provides an extremely stable and accurate reading.
        self.saadc.sample(&mut buf).await;
        self.saadc.sample(&mut buf).await;
        Ok(buf[0] as u16)
    }
}
