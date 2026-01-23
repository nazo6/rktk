pub fn init_peri() -> Peripherals {
    let p = {
        let config = {
            let mut config = embassy_nrf::config::Config::default();
            #[cfg(feature = "sd")]
            {
                use embassy_nrf::interrupt::Priority;
                config.gpiote_interrupt_priority = Priority::P2;
                config.time_interrupt_priority = Priority::P2;
            }
            config.lfclk_source = embassy_nrf::config::LfclkSource::ExternalXtal;
            config.hfclk_source = embassy_nrf::config::HfclkSource::ExternalXtal;

            config
        };
        embassy_nrf::init(config)
    };

    #[cfg(feature = "sd")]
    {
        use embassy_nrf::interrupt::{self, *};
        interrupt::USBD.set_priority(Priority::P2);
        interrupt::SPI2.set_priority(Priority::P2);
        interrupt::SPIM3.set_priority(Priority::P2);
        interrupt::UARTE0.set_priority(Priority::P2);
    }

    #[cfg(feature = "alloc")]
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 32768;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { crate::HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    p
}

use embassy_nrf::Peripherals;
#[cfg(feature = "sd")]
pub(crate) use sd::init_sd;
#[cfg(feature = "sd")]
mod sd {
    use rktk_drivers_nrf::softdevice::{
        ble::{SoftdeviceBleReporterBuilder, init_ble_server},
        flash::{SoftdeviceFlashPartition, get_typical_flash_partitions},
        init_softdevice,
    };

    pub async fn init_sd(
        spawner: embassy_executor::Spawner,
    ) -> (SoftdeviceBleReporterBuilder, SoftdeviceFlashPartition) {
        let sd = init_softdevice("negL");

        let server = init_ble_server(
            sd,
            rktk_drivers_nrf::softdevice::ble::DeviceInformation {
                manufacturer_name: Some("nazo6"),
                model_number: Some("100"),
                serial_number: Some("100"),
                ..Default::default()
            },
        );
        let (part_main, part_bond) = get_typical_flash_partitions(sd);

        rktk_drivers_nrf::softdevice::start_softdevice(spawner, sd);
        embassy_time::Timer::after_millis(200).await;

        (
            SoftdeviceBleReporterBuilder::new(spawner, sd, server, "negL", part_bond),
            part_main,
        )
    }
}
