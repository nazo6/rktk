use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use nrf_softdevice::{Flash, Softdevice};
use rktk_drivers_common::storage::flash_sequential_map::{
    sequential_storage::cache::NoCache, FlashSequentialMapStorage,
};
use static_cell::StaticCell;

pub type SharedFlash = Mutex<CriticalSectionRawMutex, Flash>;

static FLASH: StaticCell<SharedFlash> = StaticCell::new();

/// Get steal from softdevice instance.
///
/// This function must be called only once. Otherwise, it will panic.
pub fn get_flash(
    sd: &Softdevice,
) -> (
    &'static SharedFlash,
    Mutex<CriticalSectionRawMutex, NoCache>,
) {
    (
        FLASH.init(Mutex::new(nrf_softdevice::Flash::take(sd))),
        Mutex::new(NoCache::new()),
    )
}

const FLASH_START: u32 = 0x000F5000;
const FLASH_END: u32 = 0x000F5000 + 0x10000;

pub fn create_storage_driver<'a>(
    flash: &'a SharedFlash,
    cache: &'a Mutex<CriticalSectionRawMutex, NoCache>,
) -> FlashSequentialMapStorage<'a, Flash> {
    FlashSequentialMapStorage {
        flash,
        flash_range: FLASH_START..FLASH_END,
        cache,
    }
}
