use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
};

use crate::interface::{
    ble::BleDriver,
    usb::{HidReport, UsbDriver},
};

pub type ReportReceiver<'a> = Receiver<'a, CriticalSectionRawMutex, HidReport, 16>;
pub type ReportSender<'a> = Sender<'a, CriticalSectionRawMutex, HidReport, 16>;
pub type ReportChannel = Channel<CriticalSectionRawMutex, HidReport, 16>;

pub async fn start_report_task<'a, USB: UsbDriver, BT: BleDriver>(
    report_receiver: ReportReceiver<'a>,
    usb: Option<USB>,
    bt: Option<BT>,
) {
    match (usb, bt) {
        (None, None) => {
            panic!("Both USB and BLE drivers are None");
        }
        (Some(mut usb), None) => loop {
            let report = report_receiver.receive().await;
            let _ = usb.send_report(report).await;
        },
        (None, Some(mut bt)) => loop {
            let report = report_receiver.receive().await;
            let _ = bt.send_report(report).await;
        },
        (Some(mut usb), Some(mut bt)) => {
            //
        }
    }
}
