use embassy_nrf::interrupt::typelevel::{Binding, Interrupt};
use embassy_nrf::peripherals::RNG;
use embassy_nrf::rng::{InterruptHandler, Rng};
use embassy_nrf::{interrupt, rng};
use nrf_sdc::mpsl::{
    ClockInterruptHandler, HighPrioInterruptHandler, LowPrioInterruptHandler,
    MultiprotocolServiceLayer,
};
use nrf_sdc::{self as sdc, mpsl};
use rktk::singleton;
use static_cell::StaticCell;

pub use mpsl::Peripherals as MpslPeripherals;
pub use sdc::Peripherals as SdcPeripherals;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdcInitError {
    Mpsl(mpsl::Error),
    Sdc(sdc::Error),
}

#[macro_export]
macro_rules! init_sdc {
    (
        $sdc_name:ident,
        $irqs:expr,
        $rng:expr,
        mpsl: ($rtc0:expr, $timer0:expr, $temp:expr, $ppi1:expr, $ppi2:expr, $ppi3:expr),
        sdc: ($ppis1:expr,$ppis2:expr,$ppis3:expr,$ppis4:expr,$ppis5:expr,$ppis6:expr,$ppis7:expr,$ppis8:expr,$ppis9:expr,$ppis10:expr,$ppis11:expr,$ppis12:expr),
        mtu: $l2cap_mtu:expr,
        txq: $l2cap_txq:expr,
        rxq: $l2cap_rxq:expr
    ) => {
        let __mpsl_p =
            $crate::sdc::MpslPeripherals::new($rtc0, $timer0, $temp, $ppi1, $ppi2, $ppi3);
        let __sdc_p = $crate::sdc::SdcPeripherals::new(
            $ppis1, $ppis2, $ppis3, $ppis4, $ppis5, $ppis6, $ppis7, $ppis8, $ppis9, $ppis10,
            $ppis11, $ppis12,
        );
        let $sdc_name = $crate::sdc::init_sdc(
            __mpsl_p, __sdc_p, $rng, $l2cap_mtu, $l2cap_txq, $l2cap_rxq, $irqs,
        )
        .await;
    };
}

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    mpsl.run().await
}

/// Initialize softdevice controller(sdc) and starts mpsl task.
///
/// This function must be called only once.
pub async fn init_sdc<
    T: Interrupt,
    I: Binding<T, LowPrioInterruptHandler>
        + Binding<interrupt::typelevel::RADIO, HighPrioInterruptHandler>
        + Binding<interrupt::typelevel::TIMER0, HighPrioInterruptHandler>
        + Binding<interrupt::typelevel::RTC0, HighPrioInterruptHandler>
        + Binding<interrupt::typelevel::CLOCK_POWER, ClockInterruptHandler>
        + Binding<interrupt::typelevel::RNG, InterruptHandler<RNG>>
        + 'static
        + Clone,
    PR: rng::Instance,
>(
    mpsl_peripherals: mpsl::Peripherals<'static>,
    sdc_peripherals: sdc::Peripherals<'static>,
    rng: &'static mut Rng<'static, PR>,
    l2cap_mtu: u8,
    l2cap_txq: u8,
    l2cap_rxq: u8,
    irqs: I,
) -> Result<nrf_sdc::SoftdeviceController<'static>, mpsl::Error> {
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    let mpsl = MPSL.init(mpsl::MultiprotocolServiceLayer::new(
        mpsl_peripherals,
        irqs.clone(),
        lfclk_cfg,
    )?);

    embassy_executor::Spawner::for_current_executor()
        .await
        .must_spawn(mpsl_task(&*mpsl));

    let sdc_mem = singleton!(sdc::Mem::<3312>::new(), sdc::Mem::<3312>);

    let sdc = sdc::Builder::new()?
        .support_adv()?
        .support_peripheral()?
        .peripheral_count(1)?
        .buffer_cfg(l2cap_mtu, l2cap_mtu, l2cap_txq, l2cap_rxq)?
        .build(sdc_peripherals, rng, mpsl, sdc_mem)?;

    Ok(sdc)
}
