pub use embassy_sync;
pub use rktk_drivers_common::storage::flash_sequential_map::FlashSequentialMapStorage;
pub use rktk_drivers_common::storage::flash_sequential_map::sequential_storage;

#[macro_export]
macro_rules! init_storage {
    ($storage:ident, $flash:expr, $dma:expr, $size:expr) => {
        let flash = ::embassy_rp::flash::Flash::<_, _, $size>::new($flash, $dma);

        const FLASH_START: u32 = 1024 * 1024;
        const FLASH_SIZE: u32 = 2 * 1024 * 1024;

        let $storage =
            $crate::flash::FlashSequentialMapStorage::new(flash, FLASH_START, FLASH_SIZE);
    };
}
