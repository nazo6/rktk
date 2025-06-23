use core::convert::Infallible;

use embassy_rp::dma::{AnyChannel, Channel};
use embassy_rp::pio::{
    Config, FifoJoin, Instance, Pio, PioPin, ShiftConfig, ShiftDirection, StateMachine,
};
use embassy_rp::{Peripheral, PeripheralRef, clocks, into_ref};
use embassy_time::Timer;
use fixed::types::U24F8;
use fixed_macro::fixed;
use rktk::drivers::interface::rgb::{LedRgb, RgbDriver};

pub struct Ws2812Pio<'a, const MAX_LED_COUNT: usize, I: Instance> {
    dma: PeripheralRef<'a, AnyChannel>,
    sm: StateMachine<'a, I, 0>,
}

impl<'a, const MAX_LED_COUNT: usize, I: Instance> Ws2812Pio<'a, MAX_LED_COUNT, I> {
    pub fn new<'b: 'a>(
        mut pio: Pio<'a, I>,
        data_pin: impl Peripheral<P = impl PioPin> + 'b,
        dma: impl Peripheral<P = impl Channel> + 'a,
    ) -> Self {
        into_ref!(dma);

        // Setup sm0

        // prepare the PIO program
        let side_set = pio::SideSet::new(false, 1, false);
        let mut a: pio::Assembler<32> = pio::Assembler::new_with_side_set(side_set);

        const T1: u8 = 2; // start bit
        const T2: u8 = 5; // data bit
        const T3: u8 = 3; // stop bit
        const CYCLES_PER_BIT: u32 = (T1 + T2 + T3) as u32;

        let mut wrap_target = a.label();
        let mut wrap_source = a.label();
        let mut do_zero = a.label();
        a.set_with_side_set(pio::SetDestination::PINDIRS, 1, 0);
        a.bind(&mut wrap_target);
        // Do stop bit
        a.out_with_delay_and_side_set(pio::OutDestination::X, 1, T3 - 1, 0);
        // Do start bit
        a.jmp_with_delay_and_side_set(pio::JmpCondition::XIsZero, &mut do_zero, T1 - 1, 1);
        // Do data bit = 1
        a.jmp_with_delay_and_side_set(pio::JmpCondition::Always, &mut wrap_target, T2 - 1, 1);
        a.bind(&mut do_zero);
        // Do data bit = 0
        a.nop_with_delay_and_side_set(T2 - 1, 0);
        a.bind(&mut wrap_source);

        let prg = a.assemble_with_wrap(wrap_source, wrap_target);
        let mut cfg = Config::default();

        // Pin config
        let out_pin = pio.common.make_pio_pin(data_pin);
        cfg.set_out_pins(&[&out_pin]);
        cfg.set_set_pins(&[&out_pin]);

        cfg.use_program(&pio.common.load_program(&prg), &[&out_pin]);

        // Clock config, measured in kHz to avoid overflows
        // TODO CLOCK_FREQ should come from embassy_rp
        let clock_freq = U24F8::from_num(clocks::clk_sys_freq() / 1000);
        let ws2812_freq = fixed!(800: U24F8);
        let bit_freq = ws2812_freq * CYCLES_PER_BIT;
        cfg.clock_divider = clock_freq / bit_freq;

        // FIFO config
        cfg.fifo_join = FifoJoin::TxOnly;
        cfg.shift_out = ShiftConfig {
            auto_fill: true,
            threshold: 24,
            direction: ShiftDirection::Left,
        };

        pio.sm0.set_config(&cfg);
        pio.sm0.set_enable(true);

        Self {
            dma: dma.map_into(),
            sm: pio.sm0,
        }
    }
}

impl<const MAX_LED_COUNT: usize, I: Instance + 'static> RgbDriver
    for Ws2812Pio<'static, MAX_LED_COUNT, I>
{
    type Error = Infallible;

    async fn write<IT: IntoIterator<Item = LedRgb<u8>>>(
        &mut self,
        pixels: IT,
    ) -> Result<(), Self::Error> {
        let mut words = [0u32; MAX_LED_COUNT];
        for (p, w) in pixels.into_iter().zip(words.iter_mut()) {
            *w = (u32::from(p[1]) << 24) | (u32::from(p[0]) << 16) | (u32::from(p[2]) << 8);
        }

        // DMA transfer
        self.sm
            .tx()
            .dma_push(self.dma.reborrow(), &words, false)
            .await;

        Timer::after_micros(55).await;

        Ok(())
    }
}
