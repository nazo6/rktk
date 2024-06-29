use embassy_executor::SpawnToken;
use embassy_usb::class::hid::{HidReaderWriter, State};
use embassy_usb::driver::Driver;

use embassy_usb::{Builder, UsbDevice};
use rktk::interface::usb::UsbDriver;
use usbd_hid::descriptor::{
    KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor as _,
};

use crate::usb::handler::{UsbDeviceHandler, UsbRequestHandler};

use super::interface::{UsbOpts, UsbResource, UsbUserOpts};
use super::RemoteWakeupSignal;

macro_rules! singleton {
    ($val:expr, $type:ty) => {{
        static STATIC_CELL: ::static_cell::StaticCell<$type> = ::static_cell::StaticCell::new();
        STATIC_CELL.init($val)
    }};
}

pub struct HidReaderWriters<'a, D: Driver<'a>> {
    pub keyboard: HidReaderWriter<'a, D, 1, 8>,
    pub mouse: HidReaderWriter<'a, D, 1, 8>,
    pub media_key: HidReaderWriter<'a, D, 1, 8>,
}

pub struct CommonUsbDriver<D: Driver<'static>> {
    pub(super) hid: HidReaderWriters<'static, D>,
    pub(super) wakeup_signal: &'static RemoteWakeupSignal,
}

impl<D: Driver<'static>> CommonUsbDriver<D> {
    /// Creates usb device from usb driver and starts background task.
    pub async fn create_and_start<S>(
        user_opts: UsbUserOpts<'static>,
        driver: D,
        start_usb: impl FnOnce(UsbDevice<'static, D>, &'static RemoteWakeupSignal) -> SpawnToken<S>,
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

        let mut builder = Builder::new(
            opts.resource.driver,
            user_opts.config,
            opts.resource.config_descriptor,
            opts.resource.bos_descriptor,
            opts.resource.msos_descriptor,
            opts.resource.control_buf,
        );

        builder.handler(opts.device_handler);

        let keyboard_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: KeyboardReport::desc(),
                request_handler: Some(opts.kb_request_handler),
                poll_ms: user_opts.kb_poll_interval,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, 1, 8>::new(&mut builder, opts.resource.state_kb, config)
        };
        let mouse_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: MouseReport::desc(),
                request_handler: Some(opts.mouse_request_handler),
                poll_ms: user_opts.mouse_poll_interval,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, 1, 8>::new(&mut builder, opts.resource.state_mouse, config)
        };
        let media_key_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: MediaKeyboardReport::desc(),
                request_handler: Some(opts.mkb_request_handler),
                poll_ms: user_opts.kb_poll_interval,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, 1, 8>::new(&mut builder, opts.resource.state_media_key, config)
        };

        // Build the builder.
        let usb = builder.build();

        let _ = embassy_executor::Spawner::for_current_executor()
            .await
            .spawn(start_usb(usb, wakeup_signal));

        Self {
            hid: HidReaderWriters {
                keyboard: keyboard_hid,
                mouse: mouse_hid,
                media_key: media_key_hid,
            },
            wakeup_signal,
        }
    }
}

impl<D: Driver<'static>> UsbDriver for CommonUsbDriver<D> {
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
