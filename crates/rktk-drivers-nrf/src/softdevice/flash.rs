use nrf_softdevice::{Flash, Softdevice};
use rktk::utils::Mutex;
use rktk_drivers_common::storage::flash_sequential_map::{
    FlashSequentialMapStorage, sequential_storage::cache::NoCache,
};
use static_cell::StaticCell;

pub type SharedFlash = Mutex<Flash>;

static FLASH: StaticCell<SharedFlash> = StaticCell::new();

/// Take flash from softdevice instance.
///
/// This function must be called only once. Otherwise, it will panic.
pub fn get_flash(sd: &Softdevice) -> &'static SharedFlash {
    FLASH.init(Mutex::new(nrf_softdevice::Flash::take(sd)))
}

// 4kb * 160 = 640kb
const FLASH_START: u32 = 4096 * 160;
// 4kb * 3 = 12kb
const FLASH_END: u32 = FLASH_START + 4096 * 3;

pub fn create_storage_driver<'a>(
    flash: &'a SharedFlash,
    cache: &'a Mutex<NoCache>,
) -> FlashSequentialMapStorage<'a, Flash> {
    FlashSequentialMapStorage {
        flash,
        flash_range: FLASH_START..FLASH_END,
        cache,
    }
}
