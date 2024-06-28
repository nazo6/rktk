pub mod handler;

use core::sync::atomic::AtomicBool;

use embassy_futures::select::{select, Either};
use embassy_rp::{peripherals::USB, usb::Driver};
pub use embassy_usb::Config as UsbConfig;
use embassy_usb::{class::hid::State, UsbDevice};
use rktk::interface::usb::Usb;
pub use rktk_drivers_common::usb::general::*;

use crate::usb::handler::{UsbDeviceHandler, UsbRequestHandler};

pub static SUSPENDED: AtomicBool = AtomicBool::new(false);

macro_rules! singleton {
    ($val:expr, $type:ty) => {{
        static STATIC_CELL: ::static_cell::StaticCell<$type> = ::static_cell::StaticCell::new();
        STATIC_CELL.init($val)
    }};
}

pub type RemoteWakeupSignal =
    embassy_sync::signal::Signal<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, ()>;

pub struct UsbDriver {
    hid: HidReaderWriters<'static, Driver<'static, USB>>,
    wakeup_signal: &'static RemoteWakeupSignal,
}

impl UsbDriver {
    pub async fn create_and_start(
        user_opts: UsbUserOpts<'static>,
        driver: Driver<'static, USB>,
    ) -> Self {
        let wakeup_signal = singleton!(RemoteWakeupSignal::new(), RemoteWakeupSignal);
        let opts = UsbOpts {
            kb_request_handler: singleton!(UsbRequestHandler {}, UsbRequestHandler),
            mouse_request_handler: singleton!(UsbRequestHandler {}, UsbRequestHandler),
            mkb_request_handler: singleton!(UsbRequestHandler {}, UsbRequestHandler),
            device_handler: singleton!(UsbDeviceHandler::new(), UsbDeviceHandler),
            resource: UsbResource {
                driver,
                config_descriptor: singleton!([0; 256], [u8; 256]),
                bos_descriptor: singleton!([0; 256], [u8; 256]),
                msos_descriptor: singleton!([0; 256], [u8; 256]),
                control_buf: singleton!([0; 64], [u8; 64]),
                state_kb: singleton!(State::new(), State),
                state_mouse: singleton!(State::new(), State),
                state_media_key: singleton!(State::new(), State),
            },
        };
        let (hid, device) = new_usb(user_opts, opts);

        embassy_executor::Spawner::for_current_executor()
            .await
            .spawn(start_usb(device, wakeup_signal))
            .expect("Failed to start usb task");

        UsbDriver { hid, wakeup_signal }
    }
}

impl Usb for UsbDriver {
    async fn wait_ready(&mut self) {
        self.hid.keyboard.ready().await;
    }

    async fn send_report(
        &mut self,
        report: rktk::interface::usb::HidReport,
    ) -> Result<(), rktk::interface::error::RktkError> {
        match report {
            rktk::interface::usb::HidReport::Keyboard(report) => {
                let _ = self.hid.keyboard.write_serialize(&report).await;
                Ok(())
            }
            rktk::interface::usb::HidReport::MediaKeyboard(report) => {
                let _ = self.hid.media_key.write_serialize(&report).await;
                Ok(())
            }
            rktk::interface::usb::HidReport::Mouse(report) => {
                let _ = self.hid.mouse.write_serialize(&report).await;
                Ok(())
            }
        }
    }

    async fn wakeup(&mut self) -> Result<(), rktk::interface::error::RktkError> {
        self.wakeup_signal.signal(());
        Ok(())
    }
}

#[embassy_executor::task]
async fn start_usb(
    mut device: UsbDevice<'static, Driver<'static, USB>>,
    signal: &'static RemoteWakeupSignal,
) {
    loop {
        device.run_until_suspend().await;
        match select(device.wait_resume(), signal.wait()).await {
            Either::First(_) => {}
            Either::Second(_) => {
                // ref: https://github.com/rp-rs/rp-hal/blob/a1b20f3a2cc0702986c478b0e1ee666f44d66853/rp2040-hal/src/usb.rs#L494
                // https://docs.rs/rp-pac/6.0.0/rp_pac/usb/regs/struct.SieCtrl.html
                //
                // TODO: クリティカルセッションのことを考慮しないとだめかも？
                // まあそんなに同時に書き込まれることはないだろうしとりあえず…
                embassy_rp::pac::USBCTRL_REGS
                    .sie_ctrl()
                    .modify(|w| w.set_resume(true));
            }
        }
    }
}
