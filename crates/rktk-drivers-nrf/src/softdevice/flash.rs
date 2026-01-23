use embassy_embedded_hal::flash::partition::Partition;
use nrf_softdevice::{Flash, Softdevice};
use rktk::utils::{Mutex, RawMutex};
use sequential_storage::{cache::NoCache, map::MapStorage};
use static_cell::StaticCell;

pub type SoftdeviceFlashStorage = MapStorage<u8, Partition<'static, RawMutex, Flash>, NoCache>;
pub type SoftdeviceFlashPartition = Partition<'static, RawMutex, Flash>;

static FLASH: StaticCell<Mutex<Flash>> = StaticCell::new();

/// Take flash from softdevice instance.
/// When you don't need custom flash partitions, use [`get_typical_flash_partitions`] instead.
///
/// This function or [`get_typical_flash_partitions`] must be called only once. Otherwise, it will panic.
pub fn get_flash(sd: &Softdevice) -> &'static Mutex<Flash> {
    FLASH.init(Mutex::new(nrf_softdevice::Flash::take(sd)))
}

const FLASH_START: u32 = 4096 * 160;
const MAIN_FLASH_SIZE: u32 = 4096 * 3; // 4kb * 3
const BOND_FLASH_SIZE: u32 = 4096 * 2; // 4kb * 2

/// Take flash and create typical flash partitions for application and bond storage.
/// Returns (application_partition, bond_partition).
///
/// This function or [`get_flash`] must be called only once. Otherwise, it will panic.
///
/// Address        Size      Content / Region Name
/// +------------+----------+-----------------------------+
/// | 0x00000000 |          |                             |
/// |     |      |  152 KB  |  SoftDevice                 |
/// | 0x00025FFF |          |                             |
/// +------------+----------+-----------------------------+
/// | 0x00026000 |          |                             |
/// |     |      |  488 KB  |  Application                |
/// | 0x0009FFFF |          |                             |
/// +------------+----------+-----------------------------+
/// | 0x000A0000 |          |                             |
/// |     |      |   12 KB  |  Store app information      |
/// | 0x000A2FFF |          |                             |
/// +------------+----------+-----------------------------+
/// | 0x000A3000 |          |                             |
/// |     |      |    8 KB  |  Store bond information     |
/// | 0x000A4FFF |          |                             |
/// +------------+----------+-----------------------------+
///
pub fn get_typical_flash_partitions(
    sd: &Softdevice,
) -> (SoftdeviceFlashPartition, SoftdeviceFlashPartition) {
    let flash = get_flash(sd);

    let partition_main = Partition::new(flash, FLASH_START, MAIN_FLASH_SIZE);
    let partition_bond = Partition::new(flash, FLASH_START + MAIN_FLASH_SIZE, BOND_FLASH_SIZE);

    (partition_main, partition_bond)
}
