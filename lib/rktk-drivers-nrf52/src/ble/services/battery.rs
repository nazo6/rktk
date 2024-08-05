use defmt::info;
use nrf_softdevice::{
    ble::{
        gatt_server::{
            self,
            builder::ServiceBuilder,
            characteristic::{Attribute, Metadata, Presentation, Properties},
            RegisterError,
        },
        Connection,
    },
    raw, Softdevice,
};

use crate::ble::constant::{BATTERY_LEVEL, BATTERY_SERVICE};

pub struct BatteryService {
    value_handle: u16,
    cccd_handle: u16,
}

impl BatteryService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, BATTERY_SERVICE)?;

        let attr = Attribute::new(&[0u8]);
        let metadata =
            Metadata::new(Properties::new().read().notify()).presentation(Presentation {
                format: raw::BLE_GATT_CPF_FORMAT_UINT8 as u8,
                exponent: 0,  /* Value * 10 ^ 0 */
                unit: 0x27AD, /* Percentage */
                name_space: raw::BLE_GATT_CPF_NAMESPACE_BTSIG as u8,
                description: raw::BLE_GATT_CPF_NAMESPACE_DESCRIPTION_UNKNOWN as u16,
            });
        let characteristic_builder =
            service_builder.add_characteristic(BATTERY_LEVEL, attr, metadata)?;
        let characteristic_handles = characteristic_builder.build();

        let _service_handle = service_builder.build();

        Ok(BatteryService {
            value_handle: characteristic_handles.value_handle,
            cccd_handle: characteristic_handles.cccd_handle,
        })
    }

    pub fn battery_level_get(&self, sd: &Softdevice) -> Result<u8, gatt_server::GetValueError> {
        let buf = &mut [0u8];
        gatt_server::get_value(sd, self.value_handle, buf)?;
        Ok(buf[0])
    }

    pub fn battery_level_set(
        &self,
        sd: &Softdevice,
        val: u8,
    ) -> Result<(), gatt_server::SetValueError> {
        gatt_server::set_value(sd, self.value_handle, &[val])
    }
    pub fn battery_level_notify(
        &self,
        conn: &Connection,
        val: u8,
    ) -> Result<(), gatt_server::NotifyValueError> {
        gatt_server::notify_value(conn, self.value_handle, &[val])
    }

    pub fn on_write(&self, handle: u16, data: &[u8]) {
        if handle == self.cccd_handle && !data.is_empty() {
            info!("battery notifications: {}", (data[0] & 0x01) != 0);
        }
    }
}
