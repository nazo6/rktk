#![no_std]

use core::panic::PanicInfo;

use embassy_nrf::bind_interrupts;
use rktk_drivers_common::panic_utils;

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

bind_interrupts!(pub struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;
    SPI2 => embassy_nrf::spim::InterruptHandler<embassy_nrf::peripherals::SPI2>;
    TWISPI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
    UARTE0 => embassy_nrf::buffered_uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;

    // These interrupts are used for sdc (trouble)
    #[cfg(feature = "trouble")]
    RNG => embassy_nrf::rng::InterruptHandler<embassy_nrf::peripherals::RNG>;
    #[cfg(feature = "trouble")]
    EGU0_SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    #[cfg(feature = "trouble")]
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    #[cfg(feature = "trouble")]
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    #[cfg(feature = "trouble")]
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;

    // CLOCK_POWER is used for vbus detection
    //
    // If sdc is enabled, set handler for both sdc and vbus detect
    #[cfg(feature = "trouble")]
    CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler,embassy_nrf::usb::vbus_detect::InterruptHandler;
    // If sdc is not enabled, set handler only for vbus detect
    // If sd is enabled, hardware vbus detect must not be used
    #[cfg(not(any(feature = "sd", feature = "trouble")))]
    CLOCK_POWER => embassy_nrf::usb::vbus_detect::InterruptHandler;
});

pub use common::init_peri;
pub use master::start_master;
pub use slave::start_slave;
