use nrf_softdevice::ble::Uuid;

pub const DEVICE_INFORMATION: Uuid = Uuid::new_16(0x180a);
pub const BATTERY_SERVICE: Uuid = Uuid::new_16(0x180f);

pub const BATTERY_LEVEL: Uuid = Uuid::new_16(0x2a19);
pub const MODEL_NUMBER: Uuid = Uuid::new_16(0x2a24);
pub const SERIAL_NUMBER: Uuid = Uuid::new_16(0x2a25);
pub const FIRMWARE_REVISION: Uuid = Uuid::new_16(0x2a26);
pub const HARDWARE_REVISION: Uuid = Uuid::new_16(0x2a27);
pub const SOFTWARE_REVISION: Uuid = Uuid::new_16(0x2a28);
pub const MANUFACTURER_NAME: Uuid = Uuid::new_16(0x2a29);
pub const PNP_ID: Uuid = Uuid::new_16(0x2a50);

pub const HID_INFO: Uuid = Uuid::new_16(0x2a4a);
pub const REPORT_MAP: Uuid = Uuid::new_16(0x2a4b);
pub const HID_CONTROL_POINT: Uuid = Uuid::new_16(0x2a4c);
pub const HID_REPORT: Uuid = Uuid::new_16(0x2a4d);
pub const PROTOCOL_MODE: Uuid = Uuid::new_16(0x2a4e);

// Main items
pub const HIDINPUT: u8 = 0x80;
pub const HIDOUTPUT: u8 = 0x90;
pub const FEATURE: u8 = 0xb0;
pub const COLLECTION: u8 = 0xa0;
pub const END_COLLECTION: u8 = 0xc0;

// Global items
pub const USAGE_PAGE: u8 = 0x04;
pub const LOGICAL_MINIMUM: u8 = 0x14;
pub const LOGICAL_MAXIMUM: u8 = 0x24;
pub const PHYSICAL_MINIMUM: u8 = 0x34;
pub const PHYSICAL_MAXIMUM: u8 = 0x44;
pub const UNIT_EXPONENT: u8 = 0x54;
pub const UNIT: u8 = 0x64;
pub const REPORT_SIZE: u8 = 0x74; //bits
pub const REPORT_ID: u8 = 0x84;
pub const REPORT_COUNT: u8 = 0x94; //bytes
pub const PUSH: u8 = 0xa4;
pub const POP: u8 = 0xb4;

// Local items
pub const USAGE: u8 = 0x08;
pub const USAGE_MINIMUM: u8 = 0x18;
pub const USAGE_MAXIMUM: u8 = 0x28;
pub const DESIGNATOR_INDEX: u8 = 0x38;
pub const DESIGNATOR_MINIMUM: u8 = 0x48;
pub const DESIGNATOR_MAXIMUM: u8 = 0x58;
pub const STRING_INDEX: u8 = 0x78;
pub const STRING_MINIMUM: u8 = 0x88;
pub const STRING_MAXIMUM: u8 = 0x98;
pub const DELIMITER: u8 = 0xa8;

pub const KEYBOARD_ID: u8 = 0x01;
pub const MEDIA_KEYS_ID: u8 = 0x02;
