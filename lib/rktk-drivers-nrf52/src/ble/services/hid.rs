use descriptor::{BleCompositeReportType, BleKeyboardReport};
use nrf_softdevice::{
    ble::{
        gatt_server::{
            self,
            builder::ServiceBuilder,
            characteristic::{Attribute, Metadata, Properties},
            NotifyValueError, RegisterError,
        },
        Connection, SecurityMode, Uuid,
    },
    Softdevice,
};
use rktk::interface::usb::HidReport;
use ssmarshal::serialize;
use usbd_hid::descriptor::SerializedDescriptor as _;

use crate::ble::constant::*;

mod descriptor;

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
    input_mouse: u16,
    input_mouse_cccd: u16,
    input_mouse_descriptor: u16,
}

impl HidService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, Uuid::new_16(0x1812))?;

        let hid_info = service_builder.add_characteristic(
            HID_INFO,
            Attribute::new([
                0x11u8, 0x1u8,  // HID version: 1.1
                0x00u8, // Country Code
                0x01u8, // Remote wake + Normally Connectable
            ])
            .security(SecurityMode::JustWorks),
            Metadata::new(Properties::new().read()),
        )?;
        let hid_info_handle = hid_info.build();

        let report_map = service_builder.add_characteristic(
            REPORT_MAP,
            Attribute::new(BleKeyboardReport::desc()).security(SecurityMode::JustWorks),
            Metadata::new(Properties::new().read()),
        )?;
        let report_map_handle = report_map.build();

        let hid_control = service_builder.add_characteristic(
            HID_CONTROL_POINT,
            Attribute::new([0u8]).security(SecurityMode::JustWorks),
            Metadata::new(Properties::new().write_without_response()),
        )?;
        let hid_control_handle = hid_control.build();

        let protocol_mode = service_builder.add_characteristic(
            PROTOCOL_MODE,
            Attribute::new([1u8]).security(SecurityMode::JustWorks),
            Metadata::new(Properties::new().read().write_without_response()),
        )?;
        let protocol_mode_handle = protocol_mode.build();

        let mut input_keyboard = service_builder.add_characteristic(
            HID_REPORT,
            Attribute::new([0u8; 8]).security(SecurityMode::JustWorks),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let input_keyboard_desc = input_keyboard.add_descriptor(
            Uuid::new_16(0x2908),
            Attribute::new([
                BleCompositeReportType::Keyboard as u8,
                1u8, // in/out
            ])
            .security(SecurityMode::JustWorks),
        )?;
        let input_keyboard_handle = input_keyboard.build();

        let mut output_keyboard = service_builder.add_characteristic(
            HID_REPORT,
            Attribute::new([0u8; 8]).security(SecurityMode::JustWorks),
            Metadata::new(Properties::new().read().write().write_without_response()),
        )?;
        let output_keyboard_desc = output_keyboard.add_descriptor(
            Uuid::new_16(0x2908),
            Attribute::new([BleCompositeReportType::Keyboard as u8, 2u8])
                .security(SecurityMode::JustWorks),
        )?;
        let output_keyboard_handle = output_keyboard.build();

        let mut input_media_keys = service_builder.add_characteristic(
            HID_REPORT,
            Attribute::new([0u8; 2]).security(SecurityMode::JustWorks),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let input_media_keys_desc = input_media_keys.add_descriptor(
            Uuid::new_16(0x2908),
            Attribute::new([BleCompositeReportType::Media as u8, 1u8])
                .security(SecurityMode::JustWorks),
        )?;
        let input_media_keys_handle = input_media_keys.build();

        let mut input_mouse = service_builder.add_characteristic(
            HID_REPORT,
            Attribute::new([0u8; 5]).security(SecurityMode::JustWorks),
            Metadata::new(Properties::new().read().write().notify()),
        )?;
        let input_mouse_desc = input_mouse.add_descriptor(
            Uuid::new_16(0x2908),
            Attribute::new([BleCompositeReportType::Mouse as u8, 1u8])
                .security(SecurityMode::JustWorks),
        )?;
        let input_mouse_handle = input_mouse.build();

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
            input_mouse: input_mouse_handle.value_handle,
            input_mouse_cccd: input_mouse_handle.cccd_handle,
            input_mouse_descriptor: input_mouse_desc.handle(),
        })
    }

    pub fn on_write(&self, conn: &Connection, handle: u16, data: &[u8]) {
        // todo
    }

    pub fn send_report(
        &self,
        conn: &Connection,
        report: HidReport,
    ) -> Result<(), NotifyValueError> {
        match report {
            HidReport::Keyboard(r) => {
                let mut buf = [0u8; 8];
                if let Ok(n) = serialize(&mut buf, &r) {
                    gatt_server::notify_value(conn, self.input_keyboard, &buf[0..n])?;
                }
            }
            HidReport::MediaKeyboard(r) => {
                let mut buf = [0u8; 8];
                if let Ok(n) = serialize(&mut buf, &r) {
                    gatt_server::notify_value(conn, self.input_media_keys, &buf[0..n])?;
                }
            }
            HidReport::Mouse(r) => {
                let mut buf = [0u8; 8];
                if let Ok(n) = serialize(&mut buf, &r) {
                    gatt_server::notify_value(conn, self.input_mouse, &buf[0..n])?;
                }
            }
        }

        Ok(())
    }
}
