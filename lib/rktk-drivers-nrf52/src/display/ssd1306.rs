use embassy_nrf::{
    gpio::Pin,
    interrupt::typelevel::Binding,
    twim::{Config, Frequency, Instance, InterruptHandler, Twim},
    Peripheral,
};
pub use rktk_drivers_common::display::ssd1306::Ssd1306Display;
use rktk_drivers_common::display::ssd1306::Ssd1306DisplayBuilder;
use ssd1306::size::{DisplaySize, DisplaySizeAsync};

pub fn create_ssd1306<'d, T: Instance, SIZE: DisplaySize + DisplaySizeAsync>(
    twim: impl Peripheral<P = T> + 'd,
    _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    sda: impl Peripheral<P = impl Pin> + 'd,
    scl: impl Peripheral<P = impl Pin> + 'd,
    size: SIZE,
) -> Ssd1306DisplayBuilder<Twim<'d, T>, SIZE> {
    let mut i2c_config = Config::default();
    i2c_config.frequency = Frequency::K400;

    let i2c = Twim::new(twim, _irq, sda, scl, i2c_config);

    Ssd1306DisplayBuilder::new(i2c, size)
}
