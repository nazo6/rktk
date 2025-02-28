use embassy_nrf::radio::Instance;
use embassy_nrf_esb::prx::{PrxInterface, PrxTask};
use rktk::drivers::interface::{
    dongle::{DongleData, DongleDriver},
    BackgroundTask, DriverBuilderWithTask,
};

// ---- Builder -------

pub struct EsbDongleDriverBuilder<T: Instance> {
    pub prx_task: PrxTask<T, 255>,
    pub prx_interface: PrxInterface,
}

impl<T: Instance> DriverBuilderWithTask for EsbDongleDriverBuilder<T> {
    type Driver = EsbDongleDriver;

    type Error = ();

    async fn build(self) -> Result<(Self::Driver, impl BackgroundTask + 'static), Self::Error> {
        Ok((
            EsbDongleDriver {
                prx_interface: self.prx_interface,
                cnt: 0,
            },
            EsbDongleDriverTask {
                prx_task: self.prx_task,
            },
        ))
    }
}

// ---- Task ------

pub struct EsbDongleDriverTask<T: Instance> {
    prx_task: PrxTask<T, 255>,
}

impl<T: Instance> BackgroundTask for EsbDongleDriverTask<T> {
    async fn run(mut self) {
        self.prx_task.run().await;
    }
}

// ----- Driver -------

pub struct EsbDongleDriver {
    prx_interface: PrxInterface,
    cnt: usize,
}

#[derive(Debug)]
pub enum EsbDongleError {
    Esb(embassy_nrf_esb::Error),
    Deserialization(postcard::Error),
}

impl DongleDriver for EsbDongleDriver {
    type Error = EsbDongleError;

    async fn recv(&mut self) -> Result<DongleData, Self::Error> {
        let mut buf = [0; 64];
        let size = self
            .prx_interface
            .recv(&mut buf)
            .await
            .map_err(EsbDongleError::Esb)?;
        let (cnt, data): (usize, DongleData) =
            postcard::from_bytes(&buf[..size]).map_err(EsbDongleError::Deserialization)?;

        if cnt - self.cnt > 1 {
            rktk_log::warn!("Dropped packets: {} -> {}", self.cnt, cnt);
        }
        self.cnt = cnt;

        Ok(data)
    }
}
