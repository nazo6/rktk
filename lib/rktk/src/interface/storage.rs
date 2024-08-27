use core::fmt::Debug;

/// Storage driver interface
pub trait StorageDriver {
    type Error: Debug;
    async fn format(&self) -> Result<(), Self::Error>;
    async fn read<const N: usize>(&self, key: u64, buf: &mut [u8]) -> Result<(), Self::Error>;
    async fn write<const N: usize>(&self, key: u64, buf: &[u8]) -> Result<(), Self::Error>;
}

pub enum DummyStorageDriver {}
impl StorageDriver for DummyStorageDriver {
    type Error = ();
    async fn format(&self) -> Result<(), Self::Error> {
        unimplemented!()
    }
    async fn read<const N: usize>(&self, _key: u64, _buf: &mut [u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
    async fn write<const N: usize>(&self, _key: u64, _buf: &[u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
