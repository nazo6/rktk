use embassy_futures::join::join;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, pipe::Pipe};
use embassy_usb::class::cdc_acm;
use embassy_usb::class::hid::{HidReaderWriter, HidWriter};
use embassy_usb::driver::{Driver, EndpointError};
use embassy_usb::UsbDevice;
use usbd_hid::descriptor::AsInputReport;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::RemoteWakeupSignal;

pub static HID_KEYBOARD_CHANNEL: Channel<CriticalSectionRawMutex, KeyboardReport, 8> =
    Channel::new();
pub static HID_MOUSE_CHANNEL: Channel<CriticalSectionRawMutex, MouseReport, 8> = Channel::new();
pub static HID_MEDIA_KEYBOARD_CHANNEL: Channel<CriticalSectionRawMutex, MediaKeyboardReport, 8> =
    Channel::new();
pub static RRP_SEND_PIPE: Pipe<CriticalSectionRawMutex, 128> = Pipe::new();
pub static RRP_RECV_PIPE: Pipe<CriticalSectionRawMutex, 128> = Pipe::new();

// -----------------
// --- USB task  ---
// -----------------

// HACK: embassy task function cannot use generics but impl trait is allowed.
// So, as a workaround, we define a trait and implement it for the struct.
//
// FIXME: This workaround works if embassy-executor's `nightly` feature is disabled. If it is enabled, compilation fails.
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
pub async fn start_usb(
    mut device: impl UsbDeviceTrait + 'static,
    signal: &'static RemoteWakeupSignal,
) {
    loop {
        device.run_until_suspend().await;
        match embassy_futures::select::select(device.wait_resume(), signal.wait()).await {
            embassy_futures::select::Either::First(_) => {}
            embassy_futures::select::Either::Second(_) => {
                if let Err(e) = device.remote_wakeup().await {
                    rktk::log::warn!("Failed to send remote wakeup: {:?}", e);
                }
            }
        }
    }
}

// -----------------
// --- hid task  ---
// -----------------

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
impl<'d, D: Driver<'d>, const N: usize> HidWriterTrait for HidWriter<'d, D, N> {
    async fn write_serialize<IR: AsInputReport>(&mut self, r: &IR) -> Result<(), EndpointError> {
        self.write_serialize(r).await
    }
}

#[embassy_executor::task]
pub async fn hid_kb(mut device: impl HidWriterTrait + 'static) {
    loop {
        let report = HID_KEYBOARD_CHANNEL.receive().await;
        let _ = device.write_serialize(&report).await;
    }
}
#[embassy_executor::task]
pub async fn hid_mkb(mut device: impl HidWriterTrait + 'static) {
    loop {
        let report = HID_MEDIA_KEYBOARD_CHANNEL.receive().await;
        let _ = device.write_serialize(&report).await;
    }
}
#[embassy_executor::task]
pub async fn hid_mouse(mut device: impl HidWriterTrait + 'static) {
    loop {
        let report = HID_MOUSE_CHANNEL.receive().await;
        let _ = device.write_serialize(&report).await;
    }
}

// -----------------
// --- rrp task  ---
// -----------------
trait SerialWriter {
    async fn write_packet(&mut self, data: &[u8]) -> Result<(), EndpointError>;
}
impl<'d, D: Driver<'d>> SerialWriter for cdc_acm::Sender<'d, D> {
    async fn write_packet(&mut self, data: &[u8]) -> Result<(), EndpointError> {
        self.write_packet(data).await
    }
}
trait SerialReader {
    async fn read_packet(&mut self, data: &mut [u8]) -> Result<usize, EndpointError>;
    async fn wait_connection(&mut self);
}
impl<'d, D: Driver<'d>> SerialReader for cdc_acm::Receiver<'d, D> {
    async fn read_packet(&mut self, data: &mut [u8]) -> Result<usize, EndpointError> {
        self.read_packet(data).await
    }
    async fn wait_connection(&mut self) {
        self.wait_connection().await
    }
}

#[embassy_executor::task]
pub async fn rrp(mut writer: impl SerialWriter + 'static, mut reader: impl SerialReader + 'static) {
    reader.wait_connection().await;
    join(
        async {
            loop {
                let mut buf = [0u8; 64];
                let Ok(to_recv_bytes) = reader.read_packet(&mut buf).await else {
                    continue;
                };
                RRP_RECV_PIPE.write_all(&buf[..to_recv_bytes]).await;
            }
        },
        async {
            loop {
                let mut buf = [0u8; 64];
                let to_send_bytes = RRP_SEND_PIPE.read(&mut buf).await;
                let _ = writer.write_packet(&buf[..to_send_bytes]).await;
            }
        },
    )
    .await;
}
