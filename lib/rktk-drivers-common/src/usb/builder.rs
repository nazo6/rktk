use embassy_usb::class::hid::{HidReaderWriter, HidWriter, State};
use embassy_usb::driver::Driver;

use super::rrp::RrpReport;
use super::rrp::RRP_HID_BUFFER_SIZE;
use embassy_usb::Builder;
use rktk::drivers::interface::DriverBuilderWithTask;
use usbd_hid::descriptor::{
    KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor as _,
};

use crate::usb::handler::{UsbDeviceHandler, UsbRequestHandler};

use super::driver::CommonUsbDriver;
use super::{task::*, ReadySignal};
use super::{RemoteWakeupSignal, UsbOpts};

macro_rules! singleton {
    ($val:expr, $type:ty) => {{
        static STATIC_CELL: ::static_cell::StaticCell<$type> = ::static_cell::StaticCell::new();
        STATIC_CELL.init($val)
    }};
}

pub struct CommonUsbDriverBuilder<D: Driver<'static>> {
    builder: Builder<'static, D>,
    keyboard_hid: HidReaderWriter<'static, D, 1, 8>,
    mouse_hid: HidWriter<'static, D, 8>,
    media_key_hid: HidWriter<'static, D, 8>,
    wakeup_signal: &'static RemoteWakeupSignal,
    ready_signal: &'static ReadySignal,
    rrp_hid: HidReaderWriter<'static, D, RRP_HID_BUFFER_SIZE, RRP_HID_BUFFER_SIZE>,
}

impl<D: Driver<'static>> CommonUsbDriverBuilder<D> {
    pub fn new(mut opts: UsbOpts<D>) -> Self {
        let wakeup_signal = singleton!(RemoteWakeupSignal::new(), RemoteWakeupSignal);
        let ready_signal = singleton!(ReadySignal::new(), ReadySignal);

        // Required for windows compatibility.
        // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
        opts.config.device_class = 0xEF;
        opts.config.device_sub_class = 0x02;
        opts.config.device_protocol = 0x01;
        opts.config.composite_with_iads = true;

        let mut builder = Builder::new(
            opts.driver,
            opts.config,
            singleton!([0; 256], [u8; 256]),
            singleton!([0; 256], [u8; 256]),
            singleton!([0; 256], [u8; 256]),
            singleton!([0; 64], [u8; 64]),
        );

        builder.handler(singleton!(UsbDeviceHandler::new(), UsbDeviceHandler));

        let keyboard_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: KeyboardReport::desc(),
                request_handler: Some(singleton!(UsbRequestHandler {}, UsbRequestHandler)),
                poll_ms: opts.kb_poll_interval,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, 1, 8>::new(&mut builder, singleton!(State::new(), State), config)
        };
        let mouse_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: MouseReport::desc(),
                request_handler: Some(singleton!(UsbRequestHandler {}, UsbRequestHandler)),
                poll_ms: opts.mouse_poll_interval,
                max_packet_size: 64,
            };
            HidWriter::<_, 8>::new(&mut builder, singleton!(State::new(), State), config)
        };
        let media_key_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: MediaKeyboardReport::desc(),
                request_handler: Some(singleton!(UsbRequestHandler {}, UsbRequestHandler)),
                poll_ms: opts.kb_poll_interval,
                max_packet_size: 64,
            };
            HidWriter::<_, 8>::new(&mut builder, singleton!(State::new(), State), config)
        };

        let rrp_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: RrpReport::desc(),
                request_handler: Some(singleton!(UsbRequestHandler {}, UsbRequestHandler)),
                poll_ms: 1,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, RRP_HID_BUFFER_SIZE, RRP_HID_BUFFER_SIZE>::new(
                &mut builder,
                singleton!(State::new(), State),
                config,
            )
        };

        Self {
            builder,
            keyboard_hid,
            mouse_hid,
            media_key_hid,
            rrp_hid,
            wakeup_signal,
            ready_signal,
        }
    }
}

impl<D: Driver<'static> + 'static> DriverBuilderWithTask for CommonUsbDriverBuilder<D> {
    type Driver = CommonUsbDriver;

    type Error = embassy_executor::SpawnError;

    #[allow(refining_impl_trait)]
    async fn build(self) -> Result<(Self::Driver, UsbBackgroundTask<'static, D>), Self::Error> {
        let usb = self.builder.build();
        Ok((
            CommonUsbDriver {
                wakeup_signal: self.wakeup_signal,
                ready_signal: self.ready_signal,
            },
            UsbBackgroundTask {
                device: usb,
                signal: self.wakeup_signal,
                ready_signal: self.ready_signal,
                keyboard_hid: self.keyboard_hid,
                media_key_hid: self.media_key_hid,
                mouse_hid: self.mouse_hid,
                rrp_hid: self.rrp_hid,
            },
        ))
    }
}
