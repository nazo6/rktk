//! Storage driver using [`sequential_storage`] and nor flash.

use core::fmt::Debug;
use embedded_storage_async::nor_flash::{MultiwriteNorFlash, NorFlash, ReadNorFlash};
use rktk::{drivers::interface::storage::StorageDriver, utils::Mutex};
pub use sequential_storage;
use sequential_storage::{
    cache::NoCache,
    map::{fetch_item, remove_all_items, store_item},
};

pub struct FlashSequentialMapStorage<'a, F: NorFlash + ReadNorFlash + MultiwriteNorFlash> {
    pub flash_range: core::ops::Range<u32>,
    pub flash: &'a Mutex<F>,
    pub cache: &'a Mutex<NoCache>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error<E: Debug> {
    #[error("Storage error: {0:?}")]
    Storage(sequential_storage::Error<E>),
    #[error("Not found")]
    NotFound,
}

impl<E: Debug> From<sequential_storage::Error<E>> for Error<E> {
    fn from(e: sequential_storage::Error<E>) -> Self {
        Self::Storage(e)
    }
}

impl<F: NorFlash + ReadNorFlash + MultiwriteNorFlash> StorageDriver
    for FlashSequentialMapStorage<'_, F>
{
    type Error = Error<F::Error>;

    async fn format(&self) -> Result<(), Self::Error> {
        let mut flash = self.flash.lock().await;
        let mut cache = self.cache.lock().await;
        remove_all_items::<u64, _>(
            &mut *flash,
            self.flash_range.clone(),
            &mut *cache,
            &mut [0; 1024],
        )
        .await?;
        Ok(())
    }

    async fn read<const N: usize>(&self, key: u64, buf: &mut [u8]) -> Result<(), Self::Error> {
        let mut flash = self.flash.lock().await;
        let mut cache = self.cache.lock().await;
        let val: [u8; N] = fetch_item(
            &mut *flash,
            self.flash_range.clone(),
            &mut *cache,
            &mut [0; 1024],
            &key,
        )
        .await?
        .ok_or(Error::NotFound)?;
        buf.copy_from_slice(&val);
        Ok(())
    }

    async fn write<const N: usize>(&self, key: u64, buf: &[u8]) -> Result<(), Self::Error> {
        let mut flash = self.flash.lock().await;
        let mut cache = self.cache.lock().await;
        store_item(
            &mut *flash,
            self.flash_range.clone(),
            &mut *cache,
            &mut [0; 1024],
            &key,
            &buf,
        )
        .await?;
        Ok(())
    }
}
