use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use sequential_storage::cache::NoCache;

use crate::softdevice::flash::SharedFlash;

use super::{Devices, MAX_PEER_COUNT};

pub static BOND_SAVE: Signal<CriticalSectionRawMutex, Devices> = Signal::new();

const BOND_FLASH_START: u32 = 4096 * 170;
const BOND_FLASH_END: u32 = BOND_FLASH_START + 4096 * 2;

const DEVICES_MAX_SIZE: usize = 192 + 100 * MAX_PEER_COUNT;

#[allow(clippy::useless_asref)]
#[embassy_executor::task]
pub async fn bonder_save_task(flash: &'static SharedFlash) {
    // let mut cache = NoCache::new();
    // loop {
    //     let data = BOND_SAVE.wait().await;
    //     let mut buf = [0; DEVICES_MAX_SIZE];
    //     let Ok(data_slice) = postcard::to_slice(&data, &mut buf) else {
    //         rktk::print!("Failed to serialize bond map");
    //         continue;
    //     };
    //     let mut flash = flash.lock().await;
    //
    //     match sequential_storage::map::store_item::<_, &[u8], _>(
    //         &mut *flash,
    //         BOND_FLASH_START..BOND_FLASH_END,
    //         &mut cache,
    //         &mut [0; DEVICES_MAX_SIZE + 1],
    //         &0u8,
    //         &data_slice.as_ref(),
    //     )
    //     .await
    //     {
    //         Ok(_) => {
    //             rktk::log::info!("Bond map stored");
    //         }
    //         Err(e) => {
    //             rktk::print!("Failed to store bond map: {:?}", e);
    //         }
    //     }
    // }
}

pub async fn read_bond_map(flash: &SharedFlash) -> Option<Devices> {
    None
    // let mut cache = NoCache::new();
    //
    // let mut flash = flash.lock().await;
    //
    // let mut buf = [0; DEVICES_MAX_SIZE + 1];
    // let Ok(Some(data)) = sequential_storage::map::fetch_item::<_, &[u8], _>(
    //     &mut *flash,
    //     BOND_FLASH_START..BOND_FLASH_END,
    //     &mut cache,
    //     &mut buf,
    //     &0u8,
    // )
    // .await
    // else {
    //     rktk::log::warn!("Failed to read bond map");
    //     return None;
    // };
    //
    // let data = postcard::from_bytes(data)
    //     .inspect_err(|e| {
    //         rktk::log::warn!("Failed to deserialize bond map: {:?}", e);
    //     })
    //     .ok()?;
    //
    // Some(data)
}
