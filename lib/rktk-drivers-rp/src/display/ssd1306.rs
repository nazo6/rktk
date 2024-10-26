use embassy_rp::{
    i2c::{Async, I2c, Instance, InterruptHandler, SclPin, SdaPin},
    interrupt::typelevel::Binding,
    Peripheral,
};
use rktk_drivers_common::display::ssd1306::Ssd1306DisplayBuilder;
use ssd1306::size::{DisplaySize, DisplaySizeAsync};

pub fn create_ssd1306<'a, I: Instance, SIZE: DisplaySize + DisplaySizeAsync>(
    i2c: impl Peripheral<P = I> + 'a,
    _irq: impl Binding<I::Interrupt, InterruptHandler<I>>,
    sda: impl Peripheral<P = impl SdaPin<I>> + 'a,
    scl: impl Peripheral<P = impl SclPin<I>> + 'a,
    size: SIZE,
) -> Ssd1306DisplayBuilder<I2c<'a, I, Async>, SIZE> {
    let mut i2c_config = embassy_rp::i2c::Config::default();
    i2c_config.frequency = 400_000;

    let i2c = embassy_rp::i2c::I2c::new_async(i2c, scl, sda, _irq, i2c_config);

    Ssd1306DisplayBuilder::new(i2c, size)
}
