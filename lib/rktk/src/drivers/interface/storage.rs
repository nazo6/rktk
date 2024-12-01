use core::fmt::Debug;

/// Storage driver interface
pub trait StorageDriver {
    type Error: Debug;
    async fn format(&self) -> Result<(), Self::Error>;
    async fn read<const N: usize>(&self, key: u64, buf: &mut [u8]) -> Result<(), Self::Error>;
    async fn write<const N: usize>(&self, key: u64, buf: &[u8]) -> Result<(), Self::Error>;
}
