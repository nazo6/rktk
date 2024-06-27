#![no_std]

pub mod double_tap;
pub mod keyscan;
pub mod mouse;
pub mod usb;

pub mod interrupts {
    use embassy_rp::{
        bind_interrupts,
        peripherals::{I2C1, PIO0, PIO1, USB},
    };

    bind_interrupts!(pub struct Irqs {
        PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
        PIO1_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO1>;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
        I2C1_IRQ => embassy_rp::i2c::InterruptHandler<I2C1>;
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
    });
}
