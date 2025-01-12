use descriptor::{BleHidReport, BleHidReportKind};
use nrf_softdevice::ble::{gatt_server::NotifyValueError, Connection};
use ssmarshal::serialize;
use usbd_hid::descriptor::SerializedDescriptor as _;

use crate::softdevice::ble::HidReport;

mod descriptor;

pub const HID_INFO: [u8; 4] = [
    0x11u8, 0x1u8,  // HID version: 1.1
    0x00u8, // Country Code
    0x01u8, // Remote wake + Normally Connectable
];

#[nrf_softdevice::gatt_service(uuid = "1812")]
pub struct HidService {
    #[characteristic(uuid = "2A4A", security = "justworks", read, value = "HID_INFO")]
    pub hid_info: u8,

    #[characteristic(
        uuid = "2A4B", // REPORT_MAP
        security = "justworks",
        read,
        value = "BleHidReport::desc()"
    )]
    pub report_map: u8,

    #[characteristic(
        uuid = "2A4C", // CONTROL_POINT
        security = "justworks",
        read,
        value = "[0u8]"
    )]
    pub control_point: u8,

    #[characteristic(
        uuid = "2A4E", // PROTOCOL_MODE
        security = "justworks",
        read,
        value = "[1u8]"
    )]
    pub protocl_mode: u8,

    #[characteristic(
        uuid = "2A4D", // HID_REPORT
        security = "justworks",
        read,
        notify,
        value = "[0u8, 1u8]",
        // HID_REPORT_REF, Keyboard, input
        descriptor(uuid = "2908", security = "justworks", value = "[BleHidReportKind::Keyboard as u8, 1u8]")
    )]
    pub keyboard_input_report: [u8; 8],

    #[characteristic(
        uuid = "2A4D", // HID_REPORT
        security = "justworks",
        write,
        value = "[0u8, 1u8]",
        // HID_REPORT_REF, Keyboard, output
        descriptor(uuid = "2908", security = "justworks", value = "[BleHidReportKind::Keyboard as u8, 2u8]"), 
    )]
    pub keyboard_output_report: u8,

    #[characteristic(
        uuid = "2A4D", // HID_REPORT
        security = "justworks",
        read,
        notify,
        value = "[0u8; 5]",
        descriptor(uuid = "2908", security = "justworks", value = "[BleHidReportKind::Mouse as u8, 1u8]"), // HID_REPORT_REF
    )]
    pub mouse_input_report: [u8; 5],

    #[characteristic(
        uuid = "2A4D", // HID_REPORT
        security = "justworks",
        read,
        notify,
        value = "[0u8; 2]",
        descriptor(uuid = "2908", security = "justworks", value = "[BleHidReportKind::Media as u8, 1u8]"), // HID_REPORT_REF
    )]
    pub media_input_report: [u8; 2],
}

impl HidService {
    pub fn send_report(
        &self,
        conn: &Connection,
        report: HidReport,
    ) -> Result<(), NotifyValueError> {
        match report {
            HidReport::Keyboard(r) => {
                let mut buf = [0u8; 8];
                if let Ok(_n) = serialize(&mut buf, &r) {
                    self.keyboard_input_report_notify(conn, &buf)?;
                }
            }
            HidReport::Mouse(r) => {
                let mut buf = [0u8; 5];
                if let Ok(_n) = serialize(&mut buf, &r) {
                    self.mouse_input_report_notify(conn, &buf)?;
                }
            }
            HidReport::MediaKeyboard(r) => {
                let mut buf = [0u8; 2];
                if let Ok(_n) = serialize(&mut buf, &r) {
                    self.media_input_report_notify(conn, &buf)?;
                }
            }
        }

        Ok(())
    }
}
