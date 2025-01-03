use super::rrp::RrpReport;
use super::rrp::RRP_HID_BUFFER_SIZE;
use embassy_futures::join::{join, join3};
use embassy_sync::pipe::Pipe;
use embassy_usb::class::hid::{HidReaderWriter, HidWriter};
use embassy_usb::driver::Driver;
use embassy_usb::UsbDevice;
use rktk::drivers::interface::BackgroundTask;
use rktk::utils::{Channel, RawMutex};
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::{ReadySignal, RemoteWakeupSignal};

pub static HID_KEYBOARD_CHANNEL: Channel<KeyboardReport, 8> = Channel::new();
pub static HID_MOUSE_CHANNEL: Channel<MouseReport, 8> = Channel::new();
pub static HID_MEDIA_KEYBOARD_CHANNEL: Channel<MediaKeyboardReport, 8> = Channel::new();
pub static RRP_SEND_PIPE: Pipe<RawMutex, 128> = Pipe::new();
pub static RRP_RECV_PIPE: Pipe<RawMutex, 128> = Pipe::new();

pub struct UsbBackgroundTask<'d, D: Driver<'d>> {
    pub device: UsbDevice<'d, D>,
    pub signal: &'static RemoteWakeupSignal,
    pub ready_signal: &'static ReadySignal,
    pub keyboard_hid: HidReaderWriter<'d, D, 1, 8>,
    pub media_key_hid: HidWriter<'d, D, 8>,
    pub mouse_hid: HidWriter<'d, D, 8>,
    pub rrp_hid: HidReaderWriter<'d, D, RRP_HID_BUFFER_SIZE, RRP_HID_BUFFER_SIZE>,
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
            rrp(self.rrp_hid),
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
                    log::warn!("Failed to send remote wakeup: {:?}", e);
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

pub async fn rrp<'d, D: Driver<'d>>(
    rrp_hid: HidReaderWriter<'d, D, RRP_HID_BUFFER_SIZE, RRP_HID_BUFFER_SIZE>,
) {
    let (mut reader, mut writer) = rrp_hid.split();
    reader.ready().await;
    join(
        async {
            loop {
                let mut buf = [0u8; RRP_HID_BUFFER_SIZE];
                let Ok(to_recv_bytes) = reader.read(&mut buf).await else {
                    // NOTE: When usb is suspended, error is returned. We have to wait for a while to avoid busy loop in such case.
                    embassy_time::Timer::after_millis(300).await;
                    continue;
                };
                if to_recv_bytes != 32 {
                    panic!("One read should give one report. Maybe packet size is not enough?");
                }

                let len = buf[0] as usize;
                if len > 0 && len < 32 {
                    RRP_RECV_PIPE.write_all(&buf[1..=len]).await;
                } else {
                    rktk::print!("Invalid report: {:?}", &buf);
                }
            }
        },
        async {
            loop {
                let mut data = [0u8; RRP_HID_BUFFER_SIZE];
                let to_send_bytes = RRP_SEND_PIPE.read(&mut data[1..]).await;
                data[0] = to_send_bytes as u8;
                let _ = writer
                    .write_serialize(&RrpReport {
                        input_data: data,
                        output_data: [0; 32],
                    })
                    .await;
            }
        },
    )
    .await;
}
