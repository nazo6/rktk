use core::cell::{Cell, RefCell};

use nrf_softdevice::ble::{
    gatt_server::{get_sys_attrs, set_sys_attrs},
    security::{IoCapabilities, SecurityHandler},
    Connection, EncryptionInfo, IdentityKey, MasterId,
};

#[derive(Debug, Clone, Copy)]
struct Peer {
    master_id: MasterId,
    key: EncryptionInfo,
    peer_id: IdentityKey,
}

pub struct Bonder {
    peer: Cell<Option<Peer>>,
    sys_attrs: RefCell<heapless::Vec<u8, 62>>,
}

impl Default for Bonder {
    fn default() -> Self {
        Bonder {
            peer: Cell::new(None),
            sys_attrs: Default::default(),
        }
    }
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
        _conn: &Connection,
        master_id: MasterId,
        key: EncryptionInfo,
        peer_id: IdentityKey,
    ) {
    }
}

static SEC: static_cell::StaticCell<Bonder> = static_cell::StaticCell::new();

pub fn init_bonder() -> &'static Bonder {
    SEC.init(Bonder::default())
}
