use embassy_usb::class::hid::{HidReaderWriter, State};
use embassy_usb::driver::Driver;

use embassy_usb::{Builder, UsbDevice};
use rktk::interface::DriverBuilder;
use usbd_hid::descriptor::{
    KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor as _,
};

use crate::usb::handler::{UsbDeviceHandler, UsbRequestHandler};

use super::driver::{CommonUsbDriver, HidReaderWriters};
use super::{RemoteWakeupSignal, UsbOpts};

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
    type Output = CommonUsbDriver<D>;

    type Error = ();

    async fn build(self) -> Result<Self::Output, Self::Error> {
        let usb = self.builder.build();

        let _ = embassy_executor::Spawner::for_current_executor()
            .await
            .spawn(start_usb(usb, self.wakeup_signal));

        Ok(Self::Output {
            hid: self.hid,
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
