use embassy_usb::class::hid::{HidReaderWriter, HidWriter, State};
use embassy_usb::driver::Driver;
use rktk::singleton;

use super::raw_hid::{RAW_HID_BUFFER_SIZE, RawHidReport};
use super::rrp::RRP_HID_BUFFER_SIZE;
use super::rrp::RrpReport;
use embassy_usb::Builder;
use rktk::drivers::interface::DriverBuilderWithTask;
use usbd_hid::descriptor::{
    KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor as _,
};

use crate::usb::handler::UsbDeviceHandler;

use super::driver::CommonUsbDriver;
use super::{ReadySignal, task::*};
use super::{RemoteWakeupSignal, UsbOpts};

pub struct CommonUsbDriverBuilder<D: Driver<'static>> {
    builder: Builder<'static, D>,
    keyboard_hid: HidReaderWriter<'static, D, 1, 8>,
    mouse_hid: HidWriter<'static, D, 8>,
    media_key_hid: HidWriter<'static, D, 8>,
    wakeup_signal: &'static RemoteWakeupSignal,
    ready_signal: &'static ReadySignal,
    rrp_hid: HidReaderWriter<'static, D, RRP_HID_BUFFER_SIZE, RRP_HID_BUFFER_SIZE>,
    raw_hid: HidReaderWriter<'static, D, RAW_HID_BUFFER_SIZE, RAW_HID_BUFFER_SIZE>,
    #[cfg(feature = "defmtusb")]
    defmt_usb: embassy_usb::class::cdc_acm::CdcAcmClass<'static, D>,
    #[cfg(feature = "defmtusb")]
    defmt_usb_use_dtr: bool,
}

impl<D: Driver<'static>> CommonUsbDriverBuilder<D> {
    pub fn new(opts: UsbOpts<D>) -> Self {
        let wakeup_signal = singleton!(RemoteWakeupSignal::new(), RemoteWakeupSignal);
        let ready_signal = singleton!(ReadySignal::new(), ReadySignal);

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
                request_handler: None,
                poll_ms: opts.kb_poll_interval,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, 1, 8>::new(&mut builder, singleton!(State::new(), State), config)
        };
        let mouse_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: MouseReport::desc(),
                request_handler: None,
                poll_ms: opts.mouse_poll_interval,
                max_packet_size: 64,
            };
            HidWriter::<_, 8>::new(&mut builder, singleton!(State::new(), State), config)
        };
        let media_key_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: MediaKeyboardReport::desc(),
                request_handler: None,
                poll_ms: opts.kb_poll_interval,
                max_packet_size: 64,
            };
            HidWriter::<_, 8>::new(&mut builder, singleton!(State::new(), State), config)
        };

        let rrp_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: RrpReport::desc(),
                request_handler: None,
                poll_ms: 1,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, RRP_HID_BUFFER_SIZE, RRP_HID_BUFFER_SIZE>::new(
                &mut builder,
                singleton!(State::new(), State),
                config,
            )
        };

        let raw_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: RawHidReport::desc(),
                request_handler: None,
                poll_ms: 1,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, RAW_HID_BUFFER_SIZE, RAW_HID_BUFFER_SIZE>::new(
                &mut builder,
                singleton!(State::new(), State),
                config,
            )
        };

        #[cfg(feature = "defmtusb")]
        let defmt_usb = embassy_usb::class::cdc_acm::CdcAcmClass::new(
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
            rrp_hid,
            wakeup_signal,
            ready_signal,
            #[cfg(feature = "defmtusb")]
            defmt_usb,
            #[cfg(feature = "defmtusb")]
            defmt_usb_use_dtr: opts.defmt_usb_use_dtr,
            raw_hid,
        }
    }
}

impl<D: Driver<'static> + 'static> DriverBuilderWithTask for CommonUsbDriverBuilder<D> {
    type Driver = CommonUsbDriver;

    type Error = embassy_executor::SpawnError;

    #[allow(refining_impl_trait)]
    async fn build(self) -> Result<(Self::Driver, UsbBackgroundTask<'static, D>), Self::Error> {
        let mut usb = self.builder.build();

        let support_wakeup_signal = usb.remote_wakeup().await.is_ok();
        Ok((
            CommonUsbDriver {
                wakeup_signal: self.wakeup_signal,
                ready_signal: self.ready_signal,
                support_wakeup_signal,
            },
            UsbBackgroundTask {
                device: usb,
                signal: self.wakeup_signal,
                ready_signal: self.ready_signal,
                keyboard_hid: self.keyboard_hid,
                media_key_hid: self.media_key_hid,
                mouse_hid: self.mouse_hid,
                rrp_hid: self.rrp_hid,
                #[cfg(feature = "defmtusb")]
                defmt_usb: self.defmt_usb,
                #[cfg(feature = "defmtusb")]
                defmt_usb_use_dtr: self.defmt_usb_use_dtr,
                raw_hid: self.raw_hid,
            },
        ))
    }
}
