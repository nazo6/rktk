use core::{convert::Infallible, marker::PhantomData, sync::atomic::AtomicBool};

use embassy_nrf::{
    interrupt::{self, typelevel::Binding},
    pac::Interrupt,
    radio::{self},
    timer,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use esb_ng::{
    bbq2::queue::BBQueue, irq::StatePTX, peripherals::PtrTimer as _, Addresses, ConfigBuilder,
    EsbApp, EsbBuffer, EsbHeader, EsbIrq, IrqTimer,
};
use postcard::experimental::max_size::MaxSize as _;
use rktk::{
    drivers::interface::{
        ble::BleDriver, dongle::DongleData, reporter::ReporterDriver, BackgroundTask,
        DriverBuilderWithTask,
    },
    utils::Channel,
};
use rktk_log::{debug, helper::Debug2Format, warn};

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
    Option<EsbIrq<1024, 1024, DongleTimerEsb, StatePTX>>,
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

// -------- Builder ----------

static REPORT_SEND_CHAN: Channel<DongleData, 16> = Channel::new();
const MAX_PAYLOAD_SIZE: u8 = 192;

pub struct EsbReporterDriverBuilder {
    _phantom: PhantomData<()>,
}

impl EsbReporterDriverBuilder {
    pub fn new(
        _timer: DongleTimer,
        _radio: DongleRadio,
        _irqs: impl Binding<<DongleTimer as timer::Instance>::Interrupt, TimerInterruptHandler>
            + Binding<<DongleRadio as radio::Instance>::Interrupt, EsbInterruptHandler>,
    ) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl DriverBuilderWithTask for EsbReporterDriverBuilder {
    type Driver = EsbReporterDriver;

    type Error = ();

    async fn build(self) -> Result<(Self::Driver, impl BackgroundTask + 'static), Self::Error> {
        static BUFFER: EsbBuffer<1024, 1024> = EsbBuffer {
            app_to_radio_buf: BBQueue::new(),
            radio_to_app_buf: BBQueue::new(),
            timer_flag: AtomicBool::new(false),
        };
        let addresses = Addresses::default();
        let config = ConfigBuilder::default()
            .maximum_transmit_attempts(0)
            .max_payload_size(MAX_PAYLOAD_SIZE)
            .check()
            .unwrap();

        let (esb_app, esb_irq, esb_timer) = BUFFER
            .try_split(
                unsafe { DongleTimerEsb::take() },
                DONGLE_RADIO_PAC,
                addresses,
                config,
            )
            .unwrap();
        let esb_irq = esb_irq.into_ptx();
        ESB_IRQ.lock().await.replace(esb_irq);
        IRQ_TIMER.lock().await.replace(esb_timer);
        unsafe {
            cortex_m::peripheral::NVIC::unmask(Interrupt::RADIO);
            cortex_m::peripheral::NVIC::unmask(Interrupt::TIMER0);
        }

        Ok((EsbReporterDriver {}, Task { esb_app }))
    }
}

// --------- Task ----------

struct Task {
    esb_app: EsbApp<1024, 1024>,
}
impl BackgroundTask for Task {
    async fn run(self) {
        let mut cnt: u8 = 0;
        let mut pid = 0;
        let (mut s, mut r) = self.esb_app.split();
        embassy_futures::join::join(
            async move {
                loop {
                    let report = REPORT_SEND_CHAN.receive().await;
                    let mut buf = [0; DongleDataWithCnt::POSTCARD_MAX_SIZE];
                    let Ok(slice) = postcard::to_slice(&(cnt, report), &mut buf) else {
                        warn!("Postcard error");
                        continue;
                    };

                    let esb_header = EsbHeader::build()
                        .max_payload(MAX_PAYLOAD_SIZE)
                        .pid(pid)
                        .pipe(0)
                        .no_ack(false)
                        .check()
                        .unwrap();
                    let mut packet = match s.wait_grant_packet(esb_header).await {
                        Ok(p) => p,
                        Err(e) => {
                            warn!("Grant packet error: {:?}", Debug2Format(&e));
                            continue;
                        }
                    };
                    packet[..slice.len()].copy_from_slice(slice);
                    packet.commit(slice.len());
                    s.start_tx();

                    debug!("Sent report: {:?}", slice);

                    cnt = cnt.wrapping_add(1);
                    if pid == 3 {
                        pid = 0;
                    } else {
                        pid += 1;
                    }
                }
            },
            async move {
                loop {
                    let p = r.wait_read_packet().await;
                    p.release();
                }
            },
        )
        .await;
    }
}

// ----------- Driver ------------

pub struct EsbReporterDriver {}

#[derive(Debug)]
pub struct ErrorMsg(&'static str);
impl core::fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl core::error::Error for ErrorMsg {}

type DongleDataWithCnt = (usize, DongleData);

impl ReporterDriver for EsbReporterDriver {
    type Error = ErrorMsg;

    fn try_send_keyboard_report(
        &self,
        report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        REPORT_SEND_CHAN
            .try_send(DongleData::Keyboard(report.into()))
            .map_err(|_| ErrorMsg("Send error"))
    }

    fn try_send_media_keyboard_report(
        &self,
        report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        REPORT_SEND_CHAN
            .try_send(DongleData::MediaKeyboard(report.into()))
            .map_err(|_| ErrorMsg("Send error"))
    }

    fn try_send_mouse_report(
        &self,
        report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        REPORT_SEND_CHAN
            .try_send(DongleData::Mouse(report.into()))
            .map_err(|_| ErrorMsg("Send error"))
    }

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn wakeup(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl BleDriver for EsbReporterDriver {
    type Error = Infallible;

    async fn clear_bond_data(&self) -> Result<(), <Self as BleDriver>::Error> {
        Ok(())
    }
}
