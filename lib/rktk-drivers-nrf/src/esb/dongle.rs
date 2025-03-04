use core::{marker::PhantomData, sync::atomic::AtomicBool};

use embassy_nrf::{
    interrupt::{self, typelevel::Binding},
    pac::Interrupt,
    radio::{self},
    timer,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use esb_ng::{
    bbq2::queue::BBQueue, irq::StatePRX, peripherals::PtrTimer as _, EsbApp, EsbBuffer, EsbIrq,
    IrqTimer,
};
use rktk::drivers::interface::{
    dongle::{DongleData, DongleDriver},
    BackgroundTask, DriverBuilderWithTask,
};
use rktk_log::helper::Debug2Format;

use super::Config;

macro_rules! use_peripheral {
    ($radio:ident, $timer:ident, $esb_timer:ident) => {
        type DongleRadio = embassy_nrf::peripherals::$radio;
        const DONGLE_RADIO_PAC: embassy_nrf::pac::radio::Radio = embassy_nrf::pac::$radio;
        type DongleTimer = embassy_nrf::peripherals::$timer;
        type DongleTimerEsb = esb_ng::peripherals::$esb_timer;
    };
}

use_peripheral!(RADIO, TIMER0, Timer0);

static IRQ_TIMER: Mutex<CriticalSectionRawMutex, Option<IrqTimer<DongleTimerEsb>>> =
    Mutex::new(None);

pub struct TimerInterruptHandler {
    _phantom: PhantomData<()>,
}

impl interrupt::typelevel::Handler<<DongleTimer as timer::Instance>::Interrupt>
    for TimerInterruptHandler
{
    unsafe fn on_interrupt() {
        if let Ok(mut irq_timer) = IRQ_TIMER.try_lock() {
            if let Some(irq_timer) = &mut *irq_timer {
                irq_timer.timer_interrupt();
            }
        }
    }
}

static ESB_IRQ: Mutex<
    CriticalSectionRawMutex,
    Option<EsbIrq<1024, 1024, DongleTimerEsb, StatePRX>>,
> = Mutex::new(None);

pub struct EsbInterruptHandler {
    _phantom: PhantomData<()>,
}

impl interrupt::typelevel::Handler<<DongleRadio as radio::Instance>::Interrupt>
    for EsbInterruptHandler
{
    unsafe fn on_interrupt() {
        if let Ok(mut esb_irq) = ESB_IRQ.try_lock() {
            if let Some(esb_irq) = &mut *esb_irq {
                if let Err(e) = esb_irq.radio_interrupt() {
                    rktk_log::warn!("Irq error: {:?}", Debug2Format(&e));
                }
            }
        }
    }
}

// ---- Builder -------

pub struct EsbDongleDriverBuilder {
    _timer: DongleTimer,
    _radio: DongleRadio,
    config: Config,
}

impl EsbDongleDriverBuilder {
    pub fn new(
        timer: DongleTimer,
        radio: DongleRadio,
        _irqs: impl Binding<<DongleTimer as timer::Instance>::Interrupt, TimerInterruptHandler>,
        config: Config,
    ) -> Self {
        Self {
            _timer: timer,
            _radio: radio,
            config,
        }
    }
}

impl DriverBuilderWithTask for EsbDongleDriverBuilder {
    type Driver = EsbDongleDriver;

    type Error = &'static str;

    async fn build(self) -> Result<(Self::Driver, impl BackgroundTask + 'static), Self::Error> {
        static BUFFER: EsbBuffer<1024, 1024> = EsbBuffer {
            app_to_radio_buf: BBQueue::new(),
            radio_to_app_buf: BBQueue::new(),
            timer_flag: AtomicBool::new(false),
        };
        let config = self
            .config
            .config
            .max_payload_size(192)
            .check()
            .map_err(|_| "Invalid config")?;
        let (esb_app, esb_irq, esb_timer) = BUFFER
            .try_split(
                unsafe { DongleTimerEsb::take() },
                DONGLE_RADIO_PAC,
                self.config.addresses,
                config,
            )
            .map_err(|_| "Failed to initialize")?;
        let mut esb_irq = esb_irq.into_prx();
        esb_irq
            .start_receiving()
            .map_err(|_| "Failed to start receiving")?;
        ESB_IRQ.lock().await.replace(esb_irq);
        IRQ_TIMER.lock().await.replace(esb_timer);

        unsafe {
            cortex_m::peripheral::NVIC::unmask(Interrupt::RADIO);
            cortex_m::peripheral::NVIC::unmask(Interrupt::TIMER0);
        }

        Ok((
            EsbDongleDriver {
                esb: esb_app,
                cnt: 0,
            },
            EsbDongleDriverTask {},
        ))
    }
}

// ---- Task ------

pub struct EsbDongleDriverTask {}

impl BackgroundTask for EsbDongleDriverTask {
    async fn run(self) {}
}

// ----- Driver -------

pub struct EsbDongleDriver {
    esb: EsbApp<1024, 1024>,
    cnt: u8,
}

#[derive(Debug)]
pub enum EsbDongleError {
    Esb(esb_ng::Error),
    Deserialization(postcard::Error),
}

impl DongleDriver for EsbDongleDriver {
    type Error = EsbDongleError;

    async fn recv(&mut self) -> Result<DongleData, Self::Error> {
        let payload = self.esb.wait_read_packet().await;
        let (cnt, data): (u8, DongleData) =
            postcard::from_bytes(&payload).map_err(EsbDongleError::Deserialization)?;

        rktk::print!("recv:{:?}", cnt);
        payload.release();
        if cnt.wrapping_sub(self.cnt) > 1 {
            rktk_log::warn!("Packet dropped: {} -> {}", self.cnt, cnt);
        }
        self.cnt = cnt;

        Ok(data)
    }
}
