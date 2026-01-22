use embassy_futures::select::{Either, select};
use rktk::utils::Signal;
use rktk_log::{info, warn};

use crate::softdevice::flash::SoftdeviceFlashStorage;

use super::{Devices, MAX_PEER_COUNT};

pub static BOND_SAVE: Signal<Devices> = Signal::new();

pub enum BondFlashCommand {
    Clear,
}
pub static BOND_FLASH: Signal<BondFlashCommand> = Signal::new();

const DEVICES_MAX_SIZE: usize = 192 + 100 * MAX_PEER_COUNT;

#[allow(clippy::useless_asref)]
#[embassy_executor::task]
pub async fn bonder_save_task(mut storage: SoftdeviceFlashStorage) {
    let mut prev_data = None;

    loop {
        match select(BOND_FLASH.wait(), BOND_SAVE.wait()).await {
            Either::First(cmd) => match cmd {
                BondFlashCommand::Clear => {
                    let res = storage.erase_all().await;
                    rktk::print!("Erase bond data: {:?}", res);
                }
            },
            Either::Second(data) => {
                if let Some(prev_data) = &prev_data
                    && *prev_data == data
                {
                    info!("Bond data save is skipped");
                }

                let mut buf = [0; DEVICES_MAX_SIZE];
                let Ok(data_slice) = postcard::to_slice(&data, &mut buf) else {
                    rktk::print!("Failed to serialize bond map");
                    continue;
                };

                match storage
                    .store_item(&mut [0; DEVICES_MAX_SIZE + 1], &0u8, &data_slice.as_ref())
                    .await
                {
                    Ok(_) => {
                        info!("Bond map stored");
                        prev_data = Some(data);
                    }
                    Err(e) => {
                        rktk::print!("Failed to store bond map: {:?}", e);
                    }
                }
            }
        }
    }
}

pub async fn read_bond_map(storage: &mut SoftdeviceFlashStorage) -> Option<Devices> {
    let mut buf = [0; DEVICES_MAX_SIZE + 1];
    let Ok(Some(data)) = storage.fetch_item(&mut buf, &0u8).await else {
        warn!("Failed to read bond map");
        return None;
    };

    let data = postcard::from_bytes(data)
        .inspect_err(|e| {
            warn!("Failed to deserialize bond map: {:?}", e);
        })
        .ok()?;

    Some(data)
}
