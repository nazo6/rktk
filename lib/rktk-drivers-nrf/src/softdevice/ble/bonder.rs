//! Bonder (security handler). Heavily inspired by rmk's implemention (https://github.com/HaoboGu/rmk/blob/main/rmk/src/ble/nrf/bonder.rs)

use core::cell::RefCell;

use nrf_softdevice::ble::{
    gatt_server::{get_sys_attrs, set_sys_attrs},
    security::{IoCapabilities, SecurityHandler},
    Connection, EncryptionInfo, IdentityKey, MasterId,
};
use rktk_log::{info, warn};
use storage::{bonder_save_task, BOND_SAVE};

use crate::softdevice::flash::SharedFlash;

mod storage;
mod types;

pub use storage::{BondFlashCommand, BOND_FLASH};
use types::*;

const MAX_PEER_COUNT: usize = 8;

pub struct Bonder {
    devices: RefCell<Devices>,
}

impl SecurityHandler for Bonder {
    fn io_capabilities(&self) -> IoCapabilities {
        IoCapabilities::None
    }

    fn can_bond(&self, _conn: &Connection) -> bool {
        true
    }

    fn on_bonded(
        &self,
        conn: &Connection,
        master_id: MasterId,
        enc: EncryptionInfo,
        peer_id: IdentityKey,
    ) {
        let mut devices = self.devices.borrow_mut();

        devices.retain(|d| !(d.peer_id.is_match(peer_id.addr)));

        let mut sys_attrs = heapless::Vec::new();
        let capacity = sys_attrs.capacity();
        sys_attrs.resize(capacity, 0).unwrap();
        let len = get_sys_attrs(conn, &mut sys_attrs).unwrap() as u16;
        sys_attrs.truncate(usize::from(len));

        let device_data = DeviceData {
            peer_id,
            master_id,
            encryption_info: enc,
            sys_attrs: Some(sys_attrs),
        };

        if let Err(data) = devices.push(device_data) {
            devices.remove(0);
            devices.push(data).unwrap();
        }

        info!("Bonded: {:?}", master_id.ediv);

        BOND_SAVE.signal(devices.clone());
    }

    fn get_key(&self, conn: &Connection, master_id: MasterId) -> Option<EncryptionInfo> {
        let encryption_info = {
            let mut data = self.devices.borrow_mut();

            let Some(device) = data.iter_mut().find(|d| d.master_id == master_id) else {
                info!("Key not found: {:?}", master_id);
                return None;
            };

            device.encryption_info
        };

        // NOTE: Without this, the BleGattsSysAttrMissing error occurs.
        // I thought this is called automatically, but it seems not.
        // ref: https://github.com/embassy-rs/nrf-softdevice/issues/256
        self.load_sys_attrs(conn);

        Some(encryption_info)
    }

    // Receive sys_attrs and save them
    fn save_sys_attrs(&self, conn: &Connection) {
        let mut devices = self.devices.borrow_mut();

        if let Some(device) = devices
            .iter_mut()
            .find(|d| d.peer_id.is_match(conn.peer_address()))
        {
            let mut sys_attrs = heapless::Vec::new();
            sys_attrs.resize(sys_attrs.capacity(), 0).unwrap();
            let len = get_sys_attrs(conn, &mut sys_attrs).unwrap() as u16;
            sys_attrs.truncate(usize::from(len));

            // NOTE: Without this, cannot reconnect on windows
            // ref: https://github.com/embassy-rs/nrf-softdevice/issues/256
            if len > 0 {
                device.sys_attrs = Some(sys_attrs);
                BOND_SAVE.signal(devices.clone());
            } else {
                info!("Got empty sys_attrs. skipping save.");
            }
        } else {
            warn!("Failed to save sys_attrs");
        }
    }

    fn load_sys_attrs(&self, conn: &Connection) {
        let devices = self.devices.borrow();

        let _res = match devices
            .iter()
            .find(|d| d.peer_id.is_match(conn.peer_address()))
            .map(|d| &d.sys_attrs)
        {
            Some(Some(sys_attrs)) => set_sys_attrs(conn, Some(sys_attrs.as_slice())),
            _ => {
                warn!("No sys_attrs to load");
                set_sys_attrs(conn, None)
            }
        };
    }
}

static SEC: static_cell::StaticCell<Bonder> = static_cell::StaticCell::new();

pub async fn init_bonder(flash: &'static SharedFlash) -> &'static Bonder {
    embassy_executor::Spawner::for_current_executor()
        .await
        .must_spawn(bonder_save_task(flash));

    let bond_map = storage::read_bond_map(flash).await.unwrap_or_default();

    info!("Loaded {} bond info", bond_map.iter().count());

    SEC.init(Bonder {
        devices: RefCell::new(bond_map),
    })
}
