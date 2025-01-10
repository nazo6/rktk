use embassy_nrf::gpio::{Level, Output};
use rktk::{drivers::interface::system::SystemDriver, utils::Mutex};

pub struct NrfSystemDriver<'d> {
    vcc_cutoff: Option<Mutex<(Output<'d>, Level)>>,
}

impl<'d> NrfSystemDriver<'d> {
    pub fn new(vcc_cutoff: Option<(Output<'d>, Level)>) -> Self {
        Self {
            vcc_cutoff: vcc_cutoff.map(|(pin, level)| Mutex::new((pin, level))),
        }
    }
}

impl SystemDriver for NrfSystemDriver<'_> {
    #[cfg(feature = "power")]
    async fn power_off(&self) {
        {
            if let Some(vcc_cutoff) = &self.vcc_cutoff {
                let mut out = vcc_cutoff.lock().await;
                let level = out.1;
                out.0.set_level(level);
                embassy_time::Timer::after_millis(50).await;
            }
        }

        embassy_nrf::power::set_system_off();
        cortex_m::asm::udf();
    }
}
