use defmt::info;
use nrf_softdevice::{
    ble::{
        gatt_server::{
            self,
            builder::ServiceBuilder,
            characteristic::{Attribute, Metadata, Properties},
            RegisterError,
        },
        Connection, Uuid,
    },
    Softdevice,
};

use crate::ble::constant::*;

macro_rules! hid {
	($(( $($xs:tt),*)),+ $(,)?) => { &[ $( (count!($($xs)*)-1) | $($xs),* ),* ] };
}

macro_rules! count {
	() => { 0u8 };
	($x:tt $($xs:tt)*) => {1u8 + count!($($xs)*)};
}

const HID_REPORT_DESCRIPTOR: &[u8] = hid!(
    (USAGE_PAGE, 0x01), // USAGE_PAGE (Generic Desktop Ctrls)
    (USAGE, 0x06),      // USAGE (Keyboard)
    (COLLECTION, 0x01), // COLLECTION (Application)
    // ------------------------------------------------- Keyboard
    (REPORT_ID, KEYBOARD_ID), //   REPORT_ID (1)
    (USAGE_PAGE, 0x07),       //   USAGE_PAGE (Kbrd/Keypad)
    (USAGE_MINIMUM, 0xE0),    //   USAGE_MINIMUM (0xE0)
    (USAGE_MAXIMUM, 0xE7),    //   USAGE_MAXIMUM (0xE7)
    (LOGICAL_MINIMUM, 0x00),  //   LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x01),  //   Logical Maximum (1)
    (REPORT_SIZE, 0x01),      //   REPORT_SIZE (1)
    (REPORT_COUNT, 0x08),     //   REPORT_COUNT (8)
    (HIDINPUT, 0x02), //   INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 1 byte (Reserved)
    (REPORT_SIZE, 0x08), //   REPORT_SIZE (8)
    (HIDINPUT, 0x01), //   INPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (REPORT_COUNT, 0x05), //   REPORT_COUNT (5) ; 5 bits (Num lock, Caps lock, Scroll lock, Compose, Kana)
    (REPORT_SIZE, 0x01),  //   REPORT_SIZE (1)
    (USAGE_PAGE, 0x08),   //   USAGE_PAGE (LEDs)
    (USAGE_MINIMUM, 0x01), //   USAGE_MINIMUM (0x01) ; Num Lock
    (USAGE_MAXIMUM, 0x05), //   USAGE_MAXIMUM (0x05) ; Kana
    (HIDOUTPUT, 0x02), //   OUTPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 3 bits (Padding)
    (REPORT_SIZE, 0x03), //   REPORT_SIZE (3)
    (HIDOUTPUT, 0x01), //   OUTPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    (REPORT_COUNT, 0x06), //   REPORT_COUNT (6) ; 6 bytes (Keys)
    (REPORT_SIZE, 0x08), //   REPORT_SIZE(8)
    (LOGICAL_MINIMUM, 0x00), //   LOGICAL_MINIMUM(0)
    (LOGICAL_MAXIMUM, 0x65), //   LOGICAL_MAXIMUM(0x65) ; 101 keys
    (USAGE_PAGE, 0x07), //   USAGE_PAGE (Kbrd/Keypad)
    (USAGE_MINIMUM, 0x00), //   USAGE_MINIMUM (0)
    (USAGE_MAXIMUM, 0x65), //   USAGE_MAXIMUM (0x65)
    (HIDINPUT, 0x00),  //   INPUT (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (END_COLLECTION),  // END_COLLECTION
    // ------------------------------------------------- Media Keys
    (USAGE_PAGE, 0x0C),         // USAGE_PAGE (Consumer)
    (USAGE, 0x01),              // USAGE (Consumer Control)
    (COLLECTION, 0x01),         // COLLECTION (Application)
    (REPORT_ID, MEDIA_KEYS_ID), //   REPORT_ID (2)
    (USAGE_PAGE, 0x0C),         //   USAGE_PAGE (Consumer)
    (LOGICAL_MINIMUM, 0x00),    //   LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x01),    //   LOGICAL_MAXIMUM (1)
    (REPORT_SIZE, 0x01),        //   REPORT_SIZE (1)
    (REPORT_COUNT, 0x10),       //   REPORT_COUNT (16)
    (USAGE, 0xB5),              //   USAGE (Scan Next Track)     ; bit 0: 1
    (USAGE, 0xB6),              //   USAGE (Scan Previous Track) ; bit 1: 2
    (USAGE, 0xB7),              //   USAGE (Stop)                ; bit 2: 4
    (USAGE, 0xCD),              //   USAGE (Play/Pause)          ; bit 3: 8
    (USAGE, 0xE2),              //   USAGE (Mute)                ; bit 4: 16
    (USAGE, 0xE9),              //   USAGE (Volume Increment)    ; bit 5: 32
    (USAGE, 0xEA),              //   USAGE (Volume Decrement)    ; bit 6: 64
    (USAGE, 0x23, 0x02),        //   Usage (WWW Home)            ; bit 7: 128
    (USAGE, 0x94, 0x01),        //   Usage (My Computer) ; bit 0: 1
    (USAGE, 0x92, 0x01),        //   Usage (Calculator)  ; bit 1: 2
    (USAGE, 0x2A, 0x02),        //   Usage (WWW fav)     ; bit 2: 4
    (USAGE, 0x21, 0x02),        //   Usage (WWW search)  ; bit 3: 8
    (USAGE, 0x26, 0x02),        //   Usage (WWW stop)    ; bit 4: 16
    (USAGE, 0x24, 0x02),        //   Usage (WWW back)    ; bit 5: 32
    (USAGE, 0x83, 0x01),        //   Usage (Media sel)   ; bit 6: 64
    (USAGE, 0x8A, 0x01),        //   Usage (Mail)        ; bit 7: 128
    (HIDINPUT, 0x02), // INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (END_COLLECTION), // END_COLLECTION
);

#[allow(dead_code)]
pub struct HidService {
    hid_info: u16,
    report_map: u16,
    hid_control: u16,
    protocol_mode: u16,
    input_keyboard: u16,
    input_keyboard_cccd: u16,
    input_keyboard_descriptor: u16,
    output_keyboard: u16,
    output_keyboard_descriptor: u16,
    input_media_keys: u16,
    input_media_keys_cccd: u16,
    input_media_keys_descriptor: u16,
}

impl HidService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, Uuid::new_16(0x1812))?;

        let hid_info = service_builder.add_characteristic(
            HID_INFO,
            Attribute::new([0x11u8, 0x1u8, 0x00u8, 0x01u8]),
            Metadata::new(Properties::new().read()),
        )?;
        let hid_info_handle = hid_info.build();

        let report_map = service_builder.add_characteristic(
            REPORT_MAP,
            Attribute::new(HID_REPORT_DESCRIPTOR),
            Metadata::new(Properties::new().read()),
        )?;
        let report_map_handle = report_map.build();

        let hid_control = service_builder.add_characteristic(
            HID_CONTROL_POINT,
            Attribute::new([0u8]),
            Metadata::new(Properties::new().write_without_response()),
        )?;
        let hid_control_handle = hid_control.build();

        let protocol_mode = service_builder.add_characteristic(
            PROTOCOL_MODE,
            Attribute::new([1u8]),
            Metadata::new(Properties::new().read().write_without_response()),
        )?;
        let protocol_mode_handle = protocol_mode.build();

        let mut input_keyboard = service_builder.add_characteristic(
            HID_REPORT,
            Attribute::new([0u8; 8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let input_keyboard_desc = input_keyboard
            .add_descriptor(Uuid::new_16(0x2908), Attribute::new([KEYBOARD_ID, 1u8]))?; // First is ID (e.g. 1 for keyboard 2 for media keys), second is in/out
        let input_keyboard_handle = input_keyboard.build();

        let mut output_keyboard = service_builder.add_characteristic(
            HID_REPORT,
            Attribute::new([0u8; 8]),
            Metadata::new(Properties::new().read().write().write_without_response()),
        )?;
        let output_keyboard_desc = output_keyboard
            .add_descriptor(Uuid::new_16(0x2908), Attribute::new([KEYBOARD_ID, 2u8]))?; // First is ID (e.g. 1 for keyboard 2 for media keys)
        let output_keyboard_handle = output_keyboard.build();

        let mut input_media_keys = service_builder.add_characteristic(
            HID_REPORT,
            Attribute::new([0u8; 16]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let input_media_keys_desc = input_media_keys
            .add_descriptor(Uuid::new_16(0x2908), Attribute::new([MEDIA_KEYS_ID, 1u8]))?;
        let input_media_keys_handle = input_media_keys.build();

        let _service_handle = service_builder.build();

        Ok(HidService {
            hid_info: hid_info_handle.value_handle,
            report_map: report_map_handle.value_handle,
            hid_control: hid_control_handle.value_handle,
            protocol_mode: protocol_mode_handle.value_handle,
            input_keyboard: input_keyboard_handle.value_handle,
            input_keyboard_cccd: input_keyboard_handle.cccd_handle,
            input_keyboard_descriptor: input_keyboard_desc.handle(),
            output_keyboard: output_keyboard_handle.value_handle,
            output_keyboard_descriptor: output_keyboard_desc.handle(),
            input_media_keys: input_media_keys_handle.value_handle,
            input_media_keys_cccd: input_media_keys_handle.cccd_handle,
            input_media_keys_descriptor: input_media_keys_desc.handle(),
        })
    }

    pub fn on_write(&self, conn: &Connection, handle: u16, data: &[u8]) {
        let val = &[
            0, // Modifiers (Shift, Ctrl, Alt, GUI, etc.)
            0, // Reserved
            0x0E, 0, 0, 0, 0, 0, // Key code array - 0x04 is 'a' and 0x1d is 'z' - for example
        ];
        // gatt_server::notify_value(conn, self.input_keyboard_cccd, val).unwrap();
        // gatt_server::notify_value(conn, self.input_keyboard_descriptor, val).unwrap();
        if handle == self.input_keyboard_cccd {
            info!("HID input keyboard notify: {:?}", data);
        } else if handle == self.output_keyboard {
            // Fires if a keyboard output is changed - e.g. the caps lock LED
            info!("HID output keyboard: {:?}", data);

            if *data.get(0).unwrap() == 1 {
                gatt_server::notify_value(conn, self.input_keyboard, val).unwrap();
                info!("Keyboard report sent");
            } else {
                gatt_server::notify_value(conn, self.input_keyboard, &[0u8; 8]).unwrap();
                info!("Keyboard report cleared");
            }
        } else if handle == self.input_media_keys_cccd {
            info!("HID input media keys: {:?}", data);
        }
    }

    pub fn send_report(&self, conn: &Connection, report: &[u8]) {
        rktk::print!("SR: {:?}", report);

        gatt_server::notify_value(conn, self.input_keyboard, report);
    }
}
