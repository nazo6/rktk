use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::hid::{HidReaderWriter, State};
use embassy_usb::driver::Driver;

use embassy_usb::Builder;
use rktk::interface::DriverBuilder;
use usbd_hid::descriptor::{
    KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor as _,
};

use crate::usb::handler::{UsbDeviceHandler, UsbRequestHandler};

use super::driver::CommonUsbDriver;
use super::task::*;
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
    mouse_hid: HidReaderWriter<'static, D, 1, 8>,
    media_key_hid: HidReaderWriter<'static, D, 1, 8>,
    wakeup_signal: &'static RemoteWakeupSignal,
    rrp_serial: CdcAcmClass<'static, D>,
}

impl<D: Driver<'static>> CommonUsbDriverBuilder<D> {
    pub fn new(mut opts: UsbOpts<D>) -> Self {
        let wakeup_signal = singleton!(RemoteWakeupSignal::new(), RemoteWakeupSignal);

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
            HidReaderWriter::<_, 1, 8>::new(&mut builder, singleton!(State::new(), State), config)
        };
        let media_key_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: MediaKeyboardReport::desc(),
                request_handler: Some(singleton!(UsbRequestHandler {}, UsbRequestHandler)),
                poll_ms: opts.kb_poll_interval,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, 1, 8>::new(&mut builder, singleton!(State::new(), State), config)
        };

        let rrp_serial = CdcAcmClass::new(
            &mut builder,
            singleton!(
                embassy_usb::class::cdc_acm::State::new(),
                embassy_usb::class::cdc_acm::State
            ),
            64,
        );

        Self {
            builder,
            keyboard_hid,
            mouse_hid,
            media_key_hid,
            rrp_serial,
            wakeup_signal,
        }
    }
}

impl<D: Driver<'static> + 'static> DriverBuilder for CommonUsbDriverBuilder<D> {
    type Output = CommonUsbDriver;

    type Error = embassy_executor::SpawnError;

    // should be called with timeout
    async fn build(mut self) -> Result<Self::Output, Self::Error> {
        let usb = self.builder.build();

        let ex = embassy_executor::Spawner::for_current_executor().await;
        ex.spawn(start_usb(usb, self.wakeup_signal))?;

        self.keyboard_hid.ready().await;

        ex.spawn(hid_kb(self.keyboard_hid))?;
        ex.spawn(hid_mkb(self.media_key_hid))?;
        ex.spawn(hid_mouse(self.mouse_hid))?;
        let (r, w) = self.rrp_serial.split();
        ex.spawn(rrp(r, w))?;

        Ok(Self::Output {
            wakeup_signal: self.wakeup_signal,
        })
    }
}
