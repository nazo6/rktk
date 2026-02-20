#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_nrf::bind_interrupts;
use rktk::{
    config::new_rktk_opts,
    drivers::{Drivers, dummy},
    hooks::create_empty_hooks,
};
use rktk_drivers_common::panic_utils;

// ===== Global linkages =====

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use embedded_alloc::LlffHeap as Heap;

#[cfg(feature = "alloc")]
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[cfg(feature = "ble-sd")]
use nrf_softdevice as _;
use rktk_drivers_nrf::system::NrfSystemDriver;

mod keymap;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    panic_utils::save_panic_info(info);
    cortex_m::peripheral::SCB::sys_reset()
}

// ===== Irq definitions =====

bind_interrupts!(pub struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;
    SPI2 => embassy_nrf::spim::InterruptHandler<embassy_nrf::peripherals::SPI2>;
    TWISPI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
    UARTE0 => embassy_nrf::buffered_uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;
    #[cfg(feature = "ble-trouble")]
    RNG => embassy_nrf::rng::InterruptHandler<embassy_nrf::peripherals::RNG>;
    #[cfg(feature = "ble-trouble")]
    EGU0_SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    #[cfg(feature = "ble-trouble")]
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    #[cfg(feature = "ble-trouble")]
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    #[cfg(feature = "ble-trouble")]
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    #[cfg(feature = "ble-trouble")]
    CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler,embassy_nrf::usb::vbus_detect::InterruptHandler;
    #[cfg(not(any(feature = "ble-sd", feature = "ble-trouble")))]
    CLOCK_POWER => embassy_nrf::usb::vbus_detect::InterruptHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = {
        let config = embassy_nrf::config::Config::default();
        embassy_nrf::init(config)
    };

    #[cfg(feature = "alloc")]
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 32768;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { crate::HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    let spi = {
        use embassy_nrf::gpio::OutputDrive;
        use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
        use embassy_sync::mutex::Mutex;
        use rktk_drivers_nrf::mouse::paw3395;

        let mut spi_config = paw3395::recommended_spi_config();
        spi_config.sck_drive = OutputDrive::Standard;
        spi_config.mosi_drive = OutputDrive::Standard;
        spi_config.frequency = embassy_nrf::spim::Frequency::K250;

        Mutex::<ThreadModeRawMutex, _>::new(embassy_nrf::spim::Spim::new(
            p.SPI2, Irqs, p.P0_17, p.P0_22, p.P0_20, spi_config,
        ))
    };

    #[cfg(feature = "ble-trouble")]
    #[cfg_attr(feature = "_check", allow(unused_variables))]
    let trouble_ble_reporter = {
        use embassy_nrf::mode::Async;
        use rand_chacha::{ChaCha12Rng, rand_core::SeedableRng as _};
        use rktk::singleton;
        use rktk_drivers_common::trouble::reporter::{
            TroubleReporterBuilder, TroubleReporterConfig,
        };
        use rktk_drivers_nrf::init_sdc;

        let mut rng = singleton!(
            embassy_nrf::rng::Rng::new(p.RNG, Irqs),
            embassy_nrf::rng::Rng<Async>
        );
        let rng_2 = singleton!(ChaCha12Rng::from_rng(&mut rng).unwrap(), ChaCha12Rng);
        init_sdc!(
            spawner,
            sdc, Irqs, rng,
            mpsl: (p.RTC0, p.TIMER0, p.TEMP, p.PPI_CH19, p.PPI_CH30, p.PPI_CH31),
            sdc: (p.PPI_CH17, p.PPI_CH18, p.PPI_CH20, p.PPI_CH21, p.PPI_CH22, p.PPI_CH23, p.PPI_CH24, p.PPI_CH25, p.PPI_CH26, p.PPI_CH27, p.PPI_CH28, p.PPI_CH29),
            mtu: 72,
            txq: 3,
            rxq: 3
        );
        TroubleReporterBuilder::<_, _, 1, 5, 72>::new(
            sdc.unwrap(),
            rng_2,
            TroubleReporterConfig {
                advertise_name: "negL Trouble",
                peripheral_config: None,
            },
        )
    };
    cfg_if::cfg_if! {
        if #[cfg(feature = "ble-sd")] {
            let ble_builder = Some(crate::common::init_sd(spawner).await.0);
        } else if #[cfg(feature = "ble-trouble")] {
            let ble_builder = Some(trouble_ble_reporter);
        } else {
            let ble_builder = dummy::ble_builder();
        }
    }

    let usb_builder = {
        #[cfg(feature = "usb")]
        {
            use rktk_drivers_common::usb::{
                CommonUsbDriverConfig, CommonUsbReporterBuilder, UsbDriverConfig,
            };

            let embassy_driver = embassy_nrf::usb::Driver::new(
                p.USBD,
                Irqs,
                rktk_drivers_nrf::get_vbus!(spawner, Irqs),
            );
            let mut driver_config = UsbDriverConfig::new(0xc0de, 0xcafe);
            driver_config.product = Some("negL");
            let opts = CommonUsbDriverConfig::new(embassy_driver, driver_config);
            Some(CommonUsbReporterBuilder::new(opts))
        }

        #[cfg(not(feature = "usb"))]
        dummy::usb_builder()
    };

    let keyscan = {
        use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
        use embassy_nrf::gpio::{Input, Output, OutputDrive, Pull};
        use rktk_drivers_common::keyscan::shift_register_matrix::ShiftRegisterMatrix;

        let shift_register_cs = Output::new(
            p.P1_04,
            embassy_nrf::gpio::Level::High,
            OutputDrive::Standard,
        );
        let shift_register_spi_device = SpiDevice::new(&spi, shift_register_cs);

        ShiftRegisterMatrix::<_, _, _, 8, 5, 5, 8>::new(
            shift_register_spi_device,
            [
                Input::new(p.P1_15, Pull::Down), // ROW0
                Input::new(p.P1_13, Pull::Down), // ROW1
                Input::new(p.P1_11, Pull::Down), // ROW2
                Input::new(p.P0_10, Pull::Down), // ROW3
                Input::new(p.P0_09, Pull::Down), // ROW4
            ],
            |row, col| Some((row, col)),
            None,
        )
    };

    let mouse = {
        #[cfg(feature = "mouse")]
        {
            use embassy_nrf::gpio::{Output, OutputDrive};
            use rktk_drivers_common::mouse::paw3395::{self, Paw3395};
            let ball_cs = Output::new(
                p.P1_06,
                embassy_nrf::gpio::Level::High,
                OutputDrive::Standard,
            );
            use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
            Some(Paw3395::new(
                SpiDevice::new(&spi, ball_cs),
                paw3395::config::Config::default(),
            ))
        }
        #[cfg(not(feature = "mouse"))]
        dummy::mouse()
    };

    let display = {
        #[cfg(feature = "display")]
        {
            use embassy_nrf::twim::Twim;
            use rktk_drivers_common::display::ssd1306;
            use rktk_drivers_common::display::ssd1306::Ssd1306Driver;
            use rktk_drivers_common::panic_utils;

            let mut display = Ssd1306Driver::new(
                Twim::new(
                    p.TWISPI0,
                    Irqs,
                    p.P1_00,
                    p.P0_11,
                    rktk_drivers_nrf::display::ssd1306::recommended_i2c_config(),
                    &mut [],
                ),
                ssd1306::prelude::DisplaySize128x32,
                ssd1306::prelude::DisplayRotation::Rotate90,
            );
            panic_utils::display_message_if_panicked(&mut display).await;
            Some(display)
        }
        #[cfg(not(feature = "display"))]
        dummy::display()
    };

    let split = {
        #[cfg(feature = "split")]
        {
            use embassy_nrf::buffered_uarte::BufferedUarte;
            use rktk::singleton;
            use rktk_drivers_nrf::split::uart_full_duplex::UartFullDuplexSplitDriver;

            let uarte_config = embassy_nrf::uarte::Config::default();

            let (sp1, sp2) = (p.P0_08, p.P0_06);

            Some(UartFullDuplexSplitDriver::new(BufferedUarte::new(
                p.UARTE0,
                p.TIMER1,
                p.PPI_CH0,
                p.PPI_CH1,
                p.PPI_GROUP0,
                sp1,
                sp2,
                Irqs,
                uarte_config,
                singleton!([0; 256], [u8; 256]),
                singleton!([0; 256], [u8; 256]),
            )))
        }
        #[cfg(not(feature = "split"))]
        dummy::split()
    };

    let rgb = {
        #[cfg(feature = "rgb")]
        {
            Some(rktk_drivers_nrf::rgb::ws2812_pwm::Ws2812Pwm::<1024, _, _>::new(p.PWM0, p.P0_24))
        }
        #[cfg(not(feature = "rgb"))]
        dummy::rgb()
    };

    let debounce = {
        #[cfg(feature = "debounce")]
        {
            Some(rktk_drivers_common::debounce::EagerDebounceDriver::new(
                embassy_time::Duration::from_millis(10),
                true,
            ))
        }
        #[cfg(not(feature = "debounce"))]
        dummy::debounce()
    };

    let encoder = {
        #[cfg(feature = "encoder")]
        {
            use embassy_nrf::gpio::{Input, Pull};
            Some(rktk_drivers_common::encoder::GeneralEncoder::new([(
                Input::new(p.P0_02, Pull::Down),
                Input::new(p.P0_29, Pull::Down),
            )]))
        }
        #[cfg(not(feature = "encoder"))]
        dummy::encoder()
    };

    // FIXME: Implement nrf flash without softdevice
    //
    // let storage = rktk_drivers_nrf::softdevice::flash::create_storage_driver(flash, &cache);

    let drivers = Drivers {
        keyscan,
        system: NrfSystemDriver::new(None),
        mouse,
        usb_builder,
        display,
        split,
        rgb,
        storage: dummy::storage(),
        ble_builder,
        debounce,
        encoder,
    };

    rktk::task::start(
        spawner,
        drivers,
        create_empty_hooks(),
        new_rktk_opts(&keymap::KEYMAP, None),
    )
    .await;
}
