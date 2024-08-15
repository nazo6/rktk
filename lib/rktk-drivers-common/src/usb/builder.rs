use embassy_usb::class::hid::{HidReaderWriter, State};
use embassy_usb::driver::{Driver, EndpointError};

use embassy_usb::{Builder, UsbDevice};
use rktk::interface::DriverBuilder;
use usbd_hid::descriptor::{
    AsInputReport, KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor as _,
};

use crate::usb::handler::{UsbDeviceHandler, UsbRequestHandler};

use super::driver::CommonUsbDriver;
use super::{
    RemoteWakeupSignal, UsbOpts, HID_KEYBOARD_CHANNEL, HID_MEDIA_KEYBOARD_CHANNEL,
    HID_MOUSE_CHANNEL,
};

pub struct HidReaderWriters<'a, D: Driver<'a>> {
    pub keyboard: HidReaderWriter<'a, D, 1, 8>,
    pub mouse: HidReaderWriter<'a, D, 1, 8>,
    pub media_key: HidReaderWriter<'a, D, 1, 8>,
}

macro_rules! singleton {
    ($val:expr, $type:ty) => {{
        static STATIC_CELL: ::static_cell::StaticCell<$type> = ::static_cell::StaticCell::new();
        STATIC_CELL.init($val)
    }};
}

pub struct CommonUsbDriverBuilder<D: Driver<'static>> {
    builder: Builder<'static, D>,
    hid: HidReaderWriters<'static, D>,
    wakeup_signal: &'static RemoteWakeupSignal,
}

impl<D: Driver<'static>> CommonUsbDriverBuilder<D> {
    pub fn new(opts: UsbOpts<D>) -> Self {
        let wakeup_signal = singleton!(RemoteWakeupSignal::new(), RemoteWakeupSignal);

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

        Self {
            builder,
            hid: HidReaderWriters {
                keyboard: keyboard_hid,
                mouse: mouse_hid,
                media_key: media_key_hid,
            },
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

        self.hid.keyboard.ready().await;

        ex.spawn(hid_kb(self.hid.keyboard))?;
        ex.spawn(hid_mkb(self.hid.media_key))?;
        ex.spawn(hid_mouse(self.hid.mouse))?;

        Ok(Self::Output {
            wakeup_signal: self.wakeup_signal,
        })
    }
}

// this is a workaround for embassy task that cannot spawn generic task
trait UsbDeviceTrait {
    async fn run_until_suspend(&mut self);
    async fn wait_resume(&mut self);
    async fn remote_wakeup(&mut self) -> Result<(), embassy_usb::RemoteWakeupError>;
}

impl<'d, D: Driver<'d>> UsbDeviceTrait for UsbDevice<'d, D> {
    async fn run_until_suspend(&mut self) {
        self.run_until_suspend().await;
    }
    async fn wait_resume(&mut self) {
        self.wait_resume().await;
    }
    async fn remote_wakeup(&mut self) -> Result<(), embassy_usb::RemoteWakeupError> {
        self.remote_wakeup().await
    }
}

#[embassy_executor::task]
async fn start_usb(mut device: impl UsbDeviceTrait + 'static, signal: &'static RemoteWakeupSignal) {
    loop {
        device.run_until_suspend().await;
        match embassy_futures::select::select(device.wait_resume(), signal.wait()).await {
            embassy_futures::select::Either::First(_) => {}
            embassy_futures::select::Either::Second(_) => {
                if let Err(e) = device.remote_wakeup().await {
                    rktk::print!("Failed to send remote wakeup: {:?}", e);
                }
            }
        }
    }
}

trait HidWriterTrait {
    async fn write_serialize<IR: AsInputReport>(&mut self, r: &IR) -> Result<(), EndpointError>;
}
impl<'d, D: Driver<'d>, const M: usize, const N: usize> HidWriterTrait
    for HidReaderWriter<'d, D, M, N>
{
    async fn write_serialize<IR: AsInputReport>(&mut self, r: &IR) -> Result<(), EndpointError> {
        self.write_serialize(r).await
    }
}

#[embassy_executor::task]
async fn hid_kb(mut device: impl HidWriterTrait + 'static) {
    loop {
        let report = HID_KEYBOARD_CHANNEL.receive().await;
        let _ = device.write_serialize(&report).await;
    }
}
#[embassy_executor::task]
async fn hid_mkb(mut device: impl HidWriterTrait + 'static) {
    loop {
        let report = HID_MEDIA_KEYBOARD_CHANNEL.receive().await;
        let _ = device.write_serialize(&report).await;
    }
}
#[embassy_executor::task]
async fn hid_mouse(mut device: impl HidWriterTrait + 'static) {
    loop {
        let report = HID_MOUSE_CHANNEL.receive().await;
        let _ = device.write_serialize(&report).await;
    }
}
