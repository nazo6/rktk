use embassy_embedded_hal::flash::partition::Partition;
use nrf_softdevice::{Flash, Softdevice};
use rktk::utils::{Mutex, RawMutex};
use sequential_storage::{cache::NoCache, map::MapStorage};
use static_cell::StaticCell;

pub type StorageType = MapStorage<u8, Partition<'static, RawMutex, Flash>, NoCache>;
pub type PartitionFlash = Partition<'static, RawMutex, Flash>;

static FLASH: StaticCell<Mutex<Flash>> = StaticCell::new();

/// Take flash from softdevice instance.
///
/// This function must be called only once. Otherwise, it will panic.
pub fn get_flash(sd: &Softdevice) -> &'static Mutex<Flash> {
    FLASH.init(Mutex::new(nrf_softdevice::Flash::take(sd)))
}

// 4kb * 160 = 640kb
const FLASH_START: u32 = 4096 * 160;
// 4kb * 3 = 12kb
const FLASH_END: u32 = FLASH_START + 4096 * 3;
