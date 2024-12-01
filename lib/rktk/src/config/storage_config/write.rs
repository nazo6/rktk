use core::fmt::Debug;
use postcard::experimental::max_size::MaxSize as _;
use rktk_keymanager::state::config::StateConfig;

use crate::{drivers::interface::storage::StorageDriver, Layer};

use super::{ConfigKey, StorageConfigManager};

#[derive(Debug)]
pub enum ConfigWriteError<E: Debug> {
    WriteError(E),
    EncodeError(postcard::Error),
}

impl<E: Debug> From<E> for ConfigWriteError<E> {
    fn from(e: E) -> Self {
        ConfigWriteError::WriteError(e)
    }
}

impl<S: StorageDriver> StorageConfigManager<S> {
    pub async fn write_version(&self, version: u16) -> Result<(), ConfigWriteError<S::Error>> {
        let key = u64::from_le_bytes([ConfigKey::Version as u8, 0, 0, 0, 0, 0, 0, 0]);

        self.storage.write::<2>(key, &version.to_le_bytes()).await?;
        Ok(())
    }

    pub async fn write_state_config(
        &self,
        data: &StateConfig,
    ) -> Result<(), ConfigWriteError<S::Error>> {
        let key = u64::from_le_bytes([ConfigKey::StateConfig as u8, 0, 0, 0, 0, 0, 0, 0]);

        let mut buf = [0; StateConfig::POSTCARD_MAX_SIZE];
        let _slice = postcard::to_slice(data, &mut buf).map_err(ConfigWriteError::EncodeError)?;
        self.storage
            .write::<{ StateConfig::POSTCARD_MAX_SIZE }>(key, &buf)
            .await?;
        Ok(())
    }

    pub async fn write_keymap(
        &self,
        layer: u8,
        data: &Layer,
    ) -> Result<(), ConfigWriteError<S::Error>> {
        let key = u64::from_le_bytes([ConfigKey::StateKeymap as u8, layer, 0, 0, 0, 0, 0, 0]);

        let mut buf = [0; Layer::POSTCARD_MAX_SIZE];
        let _slice = postcard::to_slice(data, &mut buf).map_err(ConfigWriteError::EncodeError)?;
        self.storage
            .write::<{ Layer::POSTCARD_MAX_SIZE }>(key, &buf)
            .await?;
        Ok(())
    }
}
