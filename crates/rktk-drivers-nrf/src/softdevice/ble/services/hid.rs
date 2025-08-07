use nrf_softdevice::ble::{
    Connection,
    gatt_server::{self, NotifyValueError},
};
use ssmarshal::serialize;

use crate::softdevice::ble::{InputReport, KB_OUTPUT_LED_SIGNAL};
use descriptor::{HidKind, ReportKind, hid_desc};

mod descriptor;

pub const HID_INFO: [u8; 4] = [
    0x11u8, 0x1u8,  // HID version: 1.1
    0x00u8, // Country Code
    0x01u8, // Remote wake + Normally Connectable
];

#[nrf_softdevice::gatt_service(uuid = "1812")]
pub struct HidService {
    #[characteristic(uuid = "2A4A", security = "justworks", read, value = "HID_INFO")]
    pub hid_info: [u8; 4],

    #[characteristic(
        uuid = "2A4B", // REPORT_MAP
        security = "justworks",
        read,
        value = "hid_desc()"
    )]
    pub report_map: u8,

    #[characteristic(
        uuid = "2A4C", // CONTROL_POINT
        security = "justworks",
        write_without_response,
        value = "[0u8]"
    )]
    pub control_point: u8,

    #[characteristic(
        uuid = "2A4E", // PROTOCOL_MODE
        security = "justworks",
        read,
        write_without_response,
        value = "[1u8]"
    )]
    pub protocl_mode: u8,

    #[characteristic(
        uuid = "2A4D", // HID_REPORT
        security = "justworks",
        read,
        notify,
        // HID_REPORT_REF, Keyboard, input
        descriptor(uuid = "2908", security = "justworks", value = "[HidKind::Keyboard as u8, ReportKind::Input as u8]")
    )]
    pub keyboard_input_report: [u8; 8],

    #[characteristic(
        uuid = "2A4D", // HID_REPORT
        security = "justworks",
        read,
        write,
        write_without_response,
        // HID_REPORT_REF, Keyboard, output
        descriptor(uuid = "2908", security = "justworks", value = "[HidKind::Keyboard as u8, ReportKind::Output as u8]"), 
    )]
    pub keyboard_output_report: u8,

    #[characteristic(
        uuid = "2A4D", // HID_REPORT
        security = "justworks",
        read,
        notify,
        descriptor(uuid = "2908", security = "justworks", value = "[HidKind::Mouse as u8, ReportKind::Input as u8]"),
    )]
    pub mouse_input_report: [u8; 5],

    #[characteristic(
        uuid = "2A4D", // HID_REPORT
        security = "justworks",
        read,
        notify,
        descriptor(uuid = "2908", security = "justworks", value = "[HidKind::Media as u8, ReportKind::Input as u8]"),
    )]
    pub media_input_report: [u8; 2],
}

impl HidService {
    pub fn on_write(&self, _conn: &Connection, handle: u16, data: &[u8]) {
        if handle == self.keyboard_output_report_value_handle && !data.is_empty() {
            KB_OUTPUT_LED_SIGNAL.signal(data[0]);
        }
    }

    pub fn send_report(
        &self,
        conn: &Connection,
        report: InputReport,
    ) -> Result<(), NotifyValueError> {
        match report {
            InputReport::Keyboard(r) => {
                let mut buf = [0u8; 8];
                if let Ok(n) = serialize(&mut buf, &r) {
                    gatt_server::notify_value(
                        conn,
                        self.keyboard_input_report_value_handle,
                        &buf[0..n],
                    )?;
                }
            }
            InputReport::MediaKeyboard(r) => {
                let mut buf = [0u8; 2];
                if let Ok(n) = serialize(&mut buf, &r) {
                    gatt_server::notify_value(
                        conn,
                        self.media_input_report_value_handle,
                        &buf[0..n],
                    )?;
                }
            }
            InputReport::Mouse(r) => {
                let mut buf = [0u8; 5];
                if let Ok(n) = serialize(&mut buf, &r) {
                    gatt_server::notify_value(
                        conn,
                        self.mouse_input_report_value_handle,
                        &buf[0..n],
                    )?;
                }
            }
        }

        Ok(())
    }
}
