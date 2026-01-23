use embassy_rp::{Peri, dma::AnyChannel, flash::Flash, peripherals::FLASH};
use rktk::drivers::interface::storage::StorageDriver;
use rktk_drivers_common::storage::flash_sequential_map::FlashSequentialMapStorage;

/// Utility to initialize flash and initialize it as storage driver.
///
/// This function uses fixed flash area for storage: from 1MB to 3MB.
/// If you need different area, you can create [`Flash`] and [`FlashSequentialMapStorage`] directly.
pub fn init_storage<const SIZE: usize>(
    flash: Peri<'static, FLASH>,
    dma: Peri<'static, AnyChannel>,
) -> impl StorageDriver {
    let flash = Flash::<_, _, SIZE>::new(flash, dma);

    const FLASH_START: u32 = 1024 * 1024;
    const FLASH_SIZE: u32 = 2 * 1024 * 1024;

    FlashSequentialMapStorage::new(flash, FLASH_START, FLASH_SIZE)
}
