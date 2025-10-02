#![no_std]

use core::panic::PanicInfo;

use embassy_nrf::bind_interrupts;
use rktk::{config::keymap::Keymap, hooks::AllHooks};
use rktk_drivers_common::panic_utils;
use rktk_ksp::RktkKsp;

mod common;
mod drivers;
mod master;
mod misc;
mod slave;

// ===== Global linkages =====

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use embedded_alloc::LlffHeap as Heap;

#[cfg(feature = "alloc")]
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[cfg(feature = "sd")]
use nrf_softdevice as _;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    panic_utils::save_panic_info(info);
    cortex_m::peripheral::SCB::sys_reset()
}

// ===== Irq definitions =====

#[cfg(not(feature = "trouble"))]
bind_interrupts!(pub struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;
    SPI2 => embassy_nrf::spim::InterruptHandler<embassy_nrf::peripherals::SPI2>;
    TWISPI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
    UARTE0 => embassy_nrf::buffered_uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;
});

#[cfg(feature = "trouble")]
bind_interrupts!(pub struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;
    SPI2 => embassy_nrf::spim::InterruptHandler<embassy_nrf::peripherals::SPI2>;
    TWISPI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
    UARTE0 => embassy_nrf::buffered_uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;
    RNG => embassy_nrf::rng::InterruptHandler<embassy_nrf::peripherals::RNG>;
    EGU0_SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler;
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
});

pub struct NegMaster;
impl RktkKsp for NegMaster {
    type Irqs = Irqs;

    type Peri = embassy_nrf::Peripherals;

    type PeriConfig = ();

    type RunConfig = &'static Keymap;

    fn init_peripherals(_config: Self::PeriConfig) -> Self::Peri {
        common::init_peri()
    }

    async fn start(
        spawner: embassy_executor::Spawner,
        p: Self::Peri,
        hooks: impl AllHooks,
        config: Self::RunConfig,
    ) {
        master::start_master(spawner, p, hooks, config).await
    }
}
