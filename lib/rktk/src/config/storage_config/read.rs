use core::fmt::Debug;
use postcard::experimental::max_size::MaxSize as _;
use rktk_keymanager::state::config::StateConfig;

use crate::{drivers::interface::storage::StorageDriver, Layer};

use super::{ConfigKey, StorageConfigManager};

#[derive(Debug)]
pub enum ConfigReadError<E: Debug> {
    ReadError(E),
    DecodeError(postcard::Error),
}

impl<E: Debug> From<E> for ConfigReadError<E> {
    fn from(e: E) -> Self {
        ConfigReadError::ReadError(e)
    }
}

impl<S: StorageDriver> StorageConfigManager<S> {
    pub async fn read_version(&self) -> Result<u16, ConfigReadError<S::Error>> {
        let mut buf = [0; 2];
        let key = u64::from_le_bytes([ConfigKey::Version as u8, 0, 0, 0, 0, 0, 0, 0]);
        self.storage.read::<2>(key, &mut buf).await?;
        Ok(u16::from_le_bytes(buf))
    }

    pub async fn read_state_config(&self) -> Result<StateConfig, ConfigReadError<S::Error>> {
        let mut buf = [0; StateConfig::POSTCARD_MAX_SIZE];
        let key = u64::from_le_bytes([ConfigKey::StateConfig as u8, 0, 0, 0, 0, 0, 0, 0]);
        self.storage
            .read::<{ StateConfig::POSTCARD_MAX_SIZE }>(key, &mut buf)
            .await?;
        let res = postcard::from_bytes(&buf).map_err(ConfigReadError::DecodeError)?;
        Ok(res)
    }

    pub async fn read_keymap(&self, layer: u8) -> Result<Layer, ConfigReadError<S::Error>> {
        let mut buf = [0; Layer::POSTCARD_MAX_SIZE];
        let key = u64::from_le_bytes([ConfigKey::StateKeymap as u8, layer, 0, 0, 0, 0, 0, 0]);
        self.storage
            .read::<{ Layer::POSTCARD_MAX_SIZE }>(key, &mut buf)
            .await?;
        let res = postcard::from_bytes(&buf).map_err(ConfigReadError::DecodeError)?;
        Ok(res)
    }
}
