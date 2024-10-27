use embassy_futures::join::{join, join3};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, pipe::Pipe};
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::hid::{HidReaderWriter, HidWriter};
use embassy_usb::driver::Driver;
use embassy_usb::UsbDevice;
use rktk::interface::BackgroundTask;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::{ReadySignal, RemoteWakeupSignal};

pub static HID_KEYBOARD_CHANNEL: Channel<CriticalSectionRawMutex, KeyboardReport, 8> =
    Channel::new();
pub static HID_MOUSE_CHANNEL: Channel<CriticalSectionRawMutex, MouseReport, 8> = Channel::new();
pub static HID_MEDIA_KEYBOARD_CHANNEL: Channel<CriticalSectionRawMutex, MediaKeyboardReport, 8> =
    Channel::new();
pub static RRP_SEND_PIPE: Pipe<CriticalSectionRawMutex, 128> = Pipe::new();
pub static RRP_RECV_PIPE: Pipe<CriticalSectionRawMutex, 128> = Pipe::new();

pub struct UsbBackgroundTask<'d, D: Driver<'d>> {
    pub device: UsbDevice<'d, D>,
    pub signal: &'static RemoteWakeupSignal,
    pub ready_signal: &'static ReadySignal,
    pub keyboard_hid: HidReaderWriter<'d, D, 1, 8>,
    pub media_key_hid: HidWriter<'d, D, 8>,
    pub mouse_hid: HidWriter<'d, D, 8>,
    pub serial: CdcAcmClass<'d, D>,
}

impl<'d, D: Driver<'d>> BackgroundTask for UsbBackgroundTask<'d, D> {
    async fn run(self) {
        join3(
            usb(self.device, self.signal),
            hid(
                self.keyboard_hid,
                self.media_key_hid,
                self.mouse_hid,
                self.ready_signal,
            ),
            rrp(self.serial),
        )
        .await;
    }
}

async fn usb<'d, D: Driver<'d>>(mut device: UsbDevice<'d, D>, signal: &'static RemoteWakeupSignal) {
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

pub async fn hid<'d, D: Driver<'d>>(
    mut keyboard_hid: HidReaderWriter<'d, D, 1, 8>,
    mut media_key_hid: HidWriter<'d, D, 8>,
    mut mouse_hid: HidWriter<'d, D, 8>,
    ready_signal: &'static ReadySignal,
) {
    keyboard_hid.ready().await;
    ready_signal.signal(());

    join3(
        async move {
            loop {
                let report = HID_KEYBOARD_CHANNEL.receive().await;
                let _ = keyboard_hid.write_serialize(&report).await;
            }
        },
        async move {
            loop {
                let report = HID_MEDIA_KEYBOARD_CHANNEL.receive().await;
                let _ = media_key_hid.write_serialize(&report).await;
            }
        },
        async move {
            loop {
                let report = HID_MOUSE_CHANNEL.receive().await;
                let _ = mouse_hid.write_serialize(&report).await;
            }
        },
    )
    .await;
}

pub async fn rrp<'d, D: Driver<'d>>(serial: CdcAcmClass<'d, D>) {
    let (mut writer, mut reader) = serial.split();
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
