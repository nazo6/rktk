#![no_std]

use core::panic::PanicInfo;

use embassy_rp::bind_interrupts;
use rktk_drivers_common::panic_utils;

mod common;
mod drivers;
mod master;
mod slave;

// ===== Global linkages =====

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use embedded_alloc::LlffHeap as Heap;

#[cfg(feature = "alloc")]
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    panic_utils::save_panic_info(info);
    cortex_m::peripheral::SCB::sys_reset()
}

// ===== Irq definitions =====

bind_interrupts!(pub struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<embassy_rp::peripherals::USB>;
    UART0_IRQ => embassy_rp::uart::BufferedInterruptHandler<embassy_rp::peripherals::UART0>;
    I2C1_IRQ => embassy_rp::i2c::InterruptHandler<embassy_rp::peripherals::I2C1>;
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH0>,
        embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH1>,
        embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH2>,
        embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH3>;
});

pub use common::init_peri;
pub use master::start_master;
pub use slave::start_slave;
