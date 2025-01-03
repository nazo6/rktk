use crate::drivers::interface::storage::StorageDriver;

mod read;
mod write;

pub struct StorageConfigManager<S: StorageDriver> {
    pub storage: S,
}

pub enum ConfigKey {
    Version = 0,
    StateConfig = 1,
    StateKeymap = 2,
}

impl<S: StorageDriver> StorageConfigManager<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}
