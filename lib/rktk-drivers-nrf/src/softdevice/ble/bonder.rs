use core::cell::RefCell;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embedded_storage_async::nor_flash::{NorFlash as _, ReadNorFlash};
use nrf_softdevice::ble::{
    // gatt_server::{get_sys_attrs, set_sys_attrs},
    security::{IoCapabilities, SecurityHandler},
    Connection,
    EncryptionInfo,
    IdentityKey,
    MasterId,
};

// 4kb * 200 = 800kb point
const BOND_FLASH_ADDR: u32 = 4096 * 200;
// (10 + 17) * 8 + map overhead < 512
const BOND_FLASH_SIZE: usize = 512;

type BondMap = heapless::FnvIndexMap<[u8; 10], [u8; 17], 8>;

use crate::softdevice::flash::SharedFlash;

pub struct Bonder {
    bond_info: RefCell<BondMap>,
    // sys_attrs: RefCell<heapless::Vec<u8, 62>>,
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
        enc: EncryptionInfo,
        _peer_id: IdentityKey,
    ) {
        let mut bond_info = self.bond_info.borrow_mut();
        let key = master_id_to_key(master_id);
        let value = encryption_info_to_value(enc);
        if let Err((k, v)) = bond_info.insert(key, value) {
            let key = *bond_info.first().unwrap().0;
            bond_info.remove(&key);
            let _ = bond_info.insert(k, v);
        };
        BOND_SAVE_CHAN.signal(bond_info.clone());

        rktk::print!("Bonded (idx: {})", bond_info.iter().count());
    }

    fn get_key(&self, _conn: &Connection, master_id: MasterId) -> Option<EncryptionInfo> {
        self.bond_info
            .borrow()
            .get(&master_id_to_key(master_id))
            .copied()
            .map(encryption_info_from_value)
    }
}

static SEC: static_cell::StaticCell<Bonder> = static_cell::StaticCell::new();

pub async fn init_bonder(flash: &'static SharedFlash) -> &'static Bonder {
    embassy_executor::Spawner::for_current_executor()
        .await
        .must_spawn(bonder_save_task(flash));

    let mut buf = [0; BOND_FLASH_SIZE];
    flash
        .lock()
        .await
        .read(BOND_FLASH_ADDR, &mut buf)
        .await
        .expect("Failed to read bond info");

    let bond_info: BondMap = postcard::from_bytes(&buf).unwrap_or(BondMap::new());

    rktk::print!("Loaded {} bond info", bond_info.iter().count());

    SEC.init(Bonder {
        // sys_attrs: Default::default(),
        bond_info: RefCell::new(bond_info),
    })
}

static BOND_SAVE_CHAN: Signal<CriticalSectionRawMutex, BondMap> = Signal::new();

#[embassy_executor::task]
async fn bonder_save_task(flash: &'static SharedFlash) {
    loop {
        let data = BOND_SAVE_CHAN.wait().await;
        let mut flash = flash.lock().await;
        let mut buf = [0; BOND_FLASH_SIZE];
        let Ok(res) = postcard::to_slice(&data, &mut buf) else {
            continue;
        };
        let len = res.len();

        match flash.write(BOND_FLASH_ADDR, &buf).await {
            Ok(_) => {
                rktk::print!("Bond info saved ({} bytes)", len);
            }
            Err(e) => {
                rktk::print!("Failed to save bond info: {:?}", e);
            }
        }
    }
}

fn master_id_to_key(master_id: MasterId) -> [u8; 10] {
    let mut key = [0; 10];
    key[..8].copy_from_slice(&master_id.rand);
    key[8] = master_id.ediv as u8;
    key[9] = (master_id.ediv >> 8) as u8;

    key
}

fn encryption_info_to_value(enc: EncryptionInfo) -> [u8; 17] {
    let mut value = [0; 17];
    value[..16].copy_from_slice(&enc.ltk);
    value[16] = enc.flags;

    value
}

fn encryption_info_from_value(value: [u8; 17]) -> EncryptionInfo {
    let mut enc = EncryptionInfo::default();
    enc.ltk.copy_from_slice(&value[..16]);
    enc.flags = value[16];
    enc
}
