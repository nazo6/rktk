use core::cell::RefCell;

use nrf_softdevice::ble::{
    gatt_server::{get_sys_attrs, set_sys_attrs},
    security::{IoCapabilities, SecurityHandler},
    Connection, EncryptionInfo, IdentityKey, MasterId,
};
use storage::{bonder_save_task, BOND_SAVE};

use crate::softdevice::flash::SharedFlash;

mod storage;
mod types;

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
        rktk::log::info!("On bonded");

        let mut devices = self.devices.borrow_mut();

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

        rktk::log::info!("Bonded: {:?}", master_id.ediv);

        BOND_SAVE.signal(devices.clone());
    }

    fn get_key(&self, conn: &Connection, master_id: MasterId) -> Option<EncryptionInfo> {
        rktk::log::info!("Get key");

        let mut data = self.devices.borrow_mut();

        let Some(device) = data
            .iter_mut()
            .find(|d| d.peer_id.is_match(conn.peer_address()))
        else {
            rktk::log::info!("No peer data: {:?}", master_id.ediv);
            return None;
        };

        rktk::log::info!("Found peer data: {:?}", master_id.ediv);

        Some(device.encryption_info)
    }

    // Receive sys_attrs and save them
    fn save_sys_attrs(&self, conn: &Connection) {
        rktk::log::info!("Save sys_attrs");

        let mut devices = self.devices.borrow_mut();

        if let Some(device) = devices
            .iter_mut()
            .find(|d| d.peer_id.is_match(conn.peer_address()))
        {
            if device.sys_attrs.is_none() {
                device.sys_attrs = Some(heapless::Vec::new())
            }
            let sys_attrs = device.sys_attrs.as_mut().unwrap();
            let capacity = sys_attrs.capacity();
            sys_attrs.resize(capacity, 0).unwrap();
            let len = get_sys_attrs(conn, sys_attrs).unwrap() as u16;
            sys_attrs.truncate(usize::from(len));

            BOND_SAVE.signal(devices.clone());
        } else {
            rktk::log::warn!("Failed to save sys_attrs",);
        }
    }

    fn load_sys_attrs(&self, conn: &Connection) {
        rktk::log::info!("Load sys_attrs");

        let devices = self.devices.borrow();

        let _res = match devices
            .iter()
            .find(|d| d.peer_id.is_match(conn.peer_address()))
            .map(|d| &d.sys_attrs)
        {
            Some(Some(sys_attrs)) => set_sys_attrs(conn, Some(sys_attrs.as_slice())),
            _ => {
                rktk::log::warn!("No sys_attrs");
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

    rktk::log::info!("Loaded {} bond info", bond_map.iter().count());

    SEC.init(Bonder {
        devices: RefCell::new(bond_map),
    })
}
