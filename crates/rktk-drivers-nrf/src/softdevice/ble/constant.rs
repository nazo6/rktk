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
