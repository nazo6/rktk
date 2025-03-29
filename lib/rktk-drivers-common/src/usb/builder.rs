use embassy_usb::{
    Builder,
    class::hid::{HidReaderWriter, HidWriter, State},
    driver::Driver,
};
use rktk::{drivers::interface::usb::UsbDriverBuilder, singleton};
use usbd_hid::descriptor::{
    KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor as _,
};

use crate::usb::handler::UsbDeviceHandler;

use super::{
    ReadySignal, UsbOpts,
    driver::CommonUsbDriver,
    raw_hid::{RAW_HID_BUFFER_SIZE, RawHidReport},
    rrp::{RRP_HID_BUFFER_SIZE, RrpReport},
    task::*,
};

pub struct CommonUsbDriverBuilder<D: Driver<'static>> {
    builder: Builder<'static, D>,
    keyboard_hid: HidReaderWriter<'static, D, 1, 8>,
    mouse_hid: HidWriter<'static, D, 8>,
    media_key_hid: HidWriter<'static, D, 8>,
    #[cfg(feature = "usb-remote-wakeup")]
    wakeup_signal: &'static super::RemoteWakeupSignal,
    ready_signal: &'static ReadySignal,
    rrp_hid: HidReaderWriter<'static, D, RRP_HID_BUFFER_SIZE, RRP_HID_BUFFER_SIZE>,
    raw_hid: HidReaderWriter<'static, D, RAW_HID_BUFFER_SIZE, RAW_HID_BUFFER_SIZE>,
    #[cfg(feature = "defmt-usb")]
    defmt_usb: embassy_usb::class::cdc_acm::CdcAcmClass<'static, D>,
    #[cfg(feature = "defmt-usb")]
    defmt_usb_use_dtr: bool,
}

impl<D: Driver<'static>> CommonUsbDriverBuilder<D> {
    pub fn new(opts: UsbOpts<D>) -> Self {
        #[cfg(feature = "usb-remote-wakeup")]
        let wakeup_signal = singleton!(super::RemoteWakeupSignal::new(), super::RemoteWakeupSignal);
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

        #[cfg(feature = "defmt-usb")]
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
            #[cfg(feature = "usb-remote-wakeup")]
            wakeup_signal,
            ready_signal,
            #[cfg(feature = "defmt-usb")]
            defmt_usb,
            #[cfg(feature = "defmt-usb")]
            defmt_usb_use_dtr: opts.defmt_usb_use_dtr,
            raw_hid,
        }
    }
}

impl<D: Driver<'static> + 'static> UsbDriverBuilder for CommonUsbDriverBuilder<D> {
    type Output = CommonUsbDriver;

    type Error = embassy_executor::SpawnError;

    #[allow(refining_impl_trait)]
    async fn build(self) -> Result<(Self::Output, impl Future<Output = ()>), Self::Error> {
        let usb = self.builder.build();

        Ok((
            CommonUsbDriver {
                #[cfg(feature = "usb-remote-wakeup")]
                wakeup_signal: self.wakeup_signal,
                ready_signal: self.ready_signal,
            },
            usb_task(
                usb,
                #[cfg(feature = "usb-remote-wakeup")]
                self.wakeup_signal,
                self.ready_signal,
                self.keyboard_hid,
                self.media_key_hid,
                self.mouse_hid,
                self.rrp_hid,
                self.raw_hid,
                #[cfg(feature = "defmt-usb")]
                self.defmt_usb,
                #[cfg(feature = "defmt-usb")]
                self.defmt_usb_use_dtr,
            ),
        ))
    }
}
