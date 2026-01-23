//! Storage driver using [`sequential_storage`] and nor flash.

use core::fmt::Debug;
use embedded_storage_async::nor_flash::{MultiwriteNorFlash, NorFlash, ReadNorFlash};
use rktk::{
    drivers::interface::{Error, storage::StorageDriver},
    utils::Mutex,
};
pub use sequential_storage;
use sequential_storage::{
    cache::NoCache,
    map::{MapConfig, MapStorage},
};

// error type

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashSequentialMapStorageError<E: Debug> {
    Storage(#[cfg_attr(feature = "defmt", defmt(Debug2Format))] sequential_storage::Error<E>),
    NotFound,
}
impl<E: Debug> Error for FlashSequentialMapStorageError<E> {}

impl<E: Debug> From<sequential_storage::Error<E>> for FlashSequentialMapStorageError<E> {
    fn from(e: sequential_storage::Error<E>) -> Self {
        Self::Storage(e)
    }
}

// storage driver

pub struct FlashSequentialMapStorage<F: NorFlash + ReadNorFlash + MultiwriteNorFlash> {
    storage: Mutex<MapStorage<u64, F, NoCache>>,
}

impl<F: NorFlash + ReadNorFlash + MultiwriteNorFlash> FlashSequentialMapStorage<F> {
    pub fn new(flash: F, start_address: u32, storage_size: u32) -> Self {
        let storage = MapStorage::new(
            flash,
            MapConfig::new(start_address..start_address + storage_size),
            NoCache,
        );
        Self {
            storage: Mutex::new(storage),
        }
    }
}

impl<F: NorFlash + ReadNorFlash + MultiwriteNorFlash> StorageDriver
    for FlashSequentialMapStorage<F>
{
    type Error = FlashSequentialMapStorageError<F::Error>;

    async fn format(&self) -> Result<(), Self::Error> {
        let mut storage = self.storage.lock().await;
        storage.remove_all_items(&mut [0; 1024]).await?;
        Ok(())
    }

    async fn read<const N: usize>(&self, key: u64, buf: &mut [u8]) -> Result<(), Self::Error> {
        let mut storage = self.storage.lock().await;
        let val: [u8; N] = storage
            .fetch_item(&mut [0; 1024], &key)
            .await?
            .ok_or(FlashSequentialMapStorageError::NotFound)?;
        buf.copy_from_slice(&val);
        Ok(())
    }

    async fn write<const N: usize>(&self, key: u64, item_buf: &[u8]) -> Result<(), Self::Error> {
        let mut storage = self.storage.lock().await;
        storage.store_item(&mut [0; 1024], &key, &item_buf).await?;
        Ok(())
    }
}
