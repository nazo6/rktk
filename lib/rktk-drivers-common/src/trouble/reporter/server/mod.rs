use trouble_host::prelude::*;

mod hid;

// GATT Server definition
#[gatt_server]
pub(super) struct Server {
    pub battery_service: BatteryService,
    // pub dis: DeviceInformationService,
    pub hid_service: HidService,
}

/// Battery service
#[gatt_service(uuid = service::BATTERY)]
pub(super) struct BatteryService {
    /// Battery Level
    #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
    #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, read, value = "Battery Level")]
    #[characteristic(uuid = characteristic::BATTERY_LEVEL, read, notify, value = 10)]
    pub level: u8,
    // #[characteristic(uuid = "408813df-5dd4-1f87-ec11-cdb001100000", write, read, notify)]
    // pub status: bool,
}

// #[gatt_service(uuid = service::DEVICE_INFORMATION)]
// pub(super) struct DeviceInformationService {
//     #[characteristic(uuid = characteristic::MANUFACTURER_NAME_STRING, read)]
//     manufacturer_name: &'static str,
//     #[characteristic(uuid = characteristic::MODEL_NUMBER_STRING, read)]
//     model_number: &'static str,
//     #[characteristic(uuid = characteristic::SERIAL_NUMBER_STRING, read)]
//     serial_number: &'static str,
// }

mod hid_uuid {
    use trouble_host::prelude::BluetoothUuid16;
    pub const HID_SERVICE: BluetoothUuid16 = BluetoothUuid16::new(0x1812);
    pub const HID_INFO: BluetoothUuid16 = BluetoothUuid16::new(0x2a4a);
    pub const REPORT_MAP: BluetoothUuid16 = BluetoothUuid16::new(0x2a4b);
    pub const HID_CONTROL_POINT: BluetoothUuid16 = BluetoothUuid16::new(0x2a4c);
    pub const HID_REPORT: BluetoothUuid16 = BluetoothUuid16::new(0x2a4d);
    pub const PROTOCOL_MODE: BluetoothUuid16 = BluetoothUuid16::new(0x2a4e);
    pub const HID_REPORT_REF: BluetoothUuid16 = BluetoothUuid16::new(0x2908);
}

#[gatt_service(uuid = hid_uuid::HID_SERVICE)]
pub(super) struct HidService {
    #[characteristic(uuid = hid_uuid::HID_INFO, read)]
    pub hid_info: u16,
    #[characteristic(uuid = hid_uuid::REPORT_MAP, read)]
    pub report_map: hid::Desc,
    #[characteristic(uuid = hid_uuid::HID_CONTROL_POINT, write_without_response)]
    pub control_point: u16,
    #[characteristic(uuid = hid_uuid::PROTOCOL_MODE, read, write_without_response)]
    pub protocol_mode: u16,
    #[characteristic(uuid = hid_uuid::HID_REPORT, read, notify)]
    #[descriptor(uuid = hid_uuid::HID_REPORT_REF, read, value = [hid::BleCompositeReportType::Keyboard as u8, 2u8])]
    pub output_keyboard: [u8; 8],
}
