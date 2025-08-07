pub use embassy_sync;
pub use rktk_drivers_common::storage::flash_sequential_map::FlashSequentialMapStorage;
pub use rktk_drivers_common::storage::flash_sequential_map::sequential_storage;

#[macro_export]
macro_rules! init_storage {
    ($storage:ident, $flash:expr, $dma:expr, $size:expr) => {
        let flash = ::embassy_rp::flash::Flash::<_, _, $size>::new($flash, $dma);
        let flash = $crate::flash::embassy_sync::mutex::Mutex::new(flash);
        let cache = $crate::flash::embassy_sync::mutex::Mutex::new(
            $crate::flash::sequential_storage::cache::NoCache::new(),
        );

        const FLASH_START: u32 = 1024 * 1024;
        const FLASH_END: u32 = 3 * 1024 * 1024;

        let $storage = $crate::flash::FlashSequentialMapStorage {
            flash: &flash,
            flash_range: FLASH_START..FLASH_END,
            cache: &cache,
        };
    };
}
