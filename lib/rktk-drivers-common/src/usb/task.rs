use super::raw_hid::RawHidReport;
use super::raw_hid::RAW_HID_BUFFER_SIZE;
use super::rrp::RrpReport;
use super::rrp::RRP_HID_BUFFER_SIZE;
use embassy_futures::join::join;
use embassy_futures::join::join4;
use embassy_futures::join::join5;
use embassy_sync::pipe::Pipe;
use embassy_usb::class::hid::{HidReaderWriter, HidWriter};
use embassy_usb::driver::Driver;
use embassy_usb::UsbDevice;
use rktk::drivers::interface::BackgroundTask;
use rktk::utils::Signal;
use rktk::utils::{Channel, RawMutex};
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::{ReadySignal, RemoteWakeupSignal};

pub static HID_KEYBOARD_CHANNEL: Channel<KeyboardReport, 8> = Channel::new();
pub static HID_MOUSE_CHANNEL: Channel<MouseReport, 8> = Channel::new();
pub static HID_MEDIA_KEYBOARD_CHANNEL: Channel<MediaKeyboardReport, 8> = Channel::new();
pub static RRP_SEND_PIPE: Pipe<RawMutex, 128> = Pipe::new();
pub static RRP_RECV_PIPE: Pipe<RawMutex, 128> = Pipe::new();
pub static RAW_HID_SEND_CHANNEL: Channel<[u8; 32], 2> = Channel::new();
pub static RAW_HID_RECV_CHANNEL: Channel<[u8; 32], 2> = Channel::new();
pub static KEYBOARD_LED_SIGNAL: Signal<u8> = Signal::new();

pub struct UsbBackgroundTask<'d, D: Driver<'d>> {
    pub device: UsbDevice<'d, D>,
    pub signal: &'static RemoteWakeupSignal,
    pub ready_signal: &'static ReadySignal,
    pub keyboard_hid: HidReaderWriter<'d, D, 1, 8>,
    pub media_key_hid: HidWriter<'d, D, 8>,
    pub mouse_hid: HidWriter<'d, D, 8>,
    pub rrp_hid: HidReaderWriter<'d, D, RRP_HID_BUFFER_SIZE, RRP_HID_BUFFER_SIZE>,
    pub raw_hid: HidReaderWriter<'d, D, RAW_HID_BUFFER_SIZE, RAW_HID_BUFFER_SIZE>,
    #[cfg(feature = "defmtusb")]
    pub defmt_usb: embassy_usb::class::cdc_acm::CdcAcmClass<'d, D>,
    #[cfg(feature = "defmtusb")]
    pub defmt_usb_use_dtr: bool,
}

impl<'d, D: Driver<'d>> BackgroundTask for UsbBackgroundTask<'d, D> {
    async fn run(self) {
        join5(
            usb(self.device, self.signal),
            hid(
                self.keyboard_hid,
                self.media_key_hid,
                self.mouse_hid,
                self.ready_signal,
            ),
            raw_hid(self.raw_hid),
            rrp(self.rrp_hid),
            async move {
                #[cfg(feature = "defmtusb")]
                {
                    let (sender, _) = self.defmt_usb.split();
                    super::defmtusb::logger(sender, 64, self.defmt_usb_use_dtr).await
                }
            },
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
                    rktk_log::warn!("Failed to send remote wakeup: {:?}", e);
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
    let (mut keyboard_reader, mut keyboard_writer) = keyboard_hid.split();

    ready_signal.signal(());

    join4(
        async move {
            loop {
                let report = HID_KEYBOARD_CHANNEL.receive().await;
                let _ = keyboard_writer.write_serialize(&report).await;
            }
        },
        async move {
            loop {
                let mut buf = [0];
                let Ok(_) = keyboard_reader.read(&mut buf).await else {
                    embassy_time::Timer::after_millis(300).await;
                    continue;
                };
                KEYBOARD_LED_SIGNAL.signal(buf[0]);
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

pub async fn raw_hid<'d, D: Driver<'d>>(
    raw_hid: HidReaderWriter<'d, D, RAW_HID_BUFFER_SIZE, RAW_HID_BUFFER_SIZE>,
) {
    let (mut reader, mut writer) = raw_hid.split();
    reader.ready().await;

    join(
        async {
            loop {
                let mut buf = [0u8; RAW_HID_BUFFER_SIZE];
                let Ok(to_recv_bytes) = reader.read(&mut buf).await else {
                    // NOTE: When usb is suspended, error is returned. We have to wait for a while to avoid busy loop in such case.
                    embassy_time::Timer::after_millis(300).await;
                    continue;
                };
                if to_recv_bytes != 32 {
                    panic!("One read should give one report. Maybe packet size is not enough?");
                }
                RAW_HID_RECV_CHANNEL.send(buf).await;
            }
        },
        async {
            loop {
                let data = RAW_HID_SEND_CHANNEL.receive().await;
                let _ = writer
                    .write_serialize(&RawHidReport {
                        input_data: data,
                        output_data: [0; 32],
                    })
                    .await;
            }
        },
    )
    .await;
}
