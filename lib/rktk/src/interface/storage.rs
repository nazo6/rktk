pub use ekv;
use ekv::Database;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
pub use embedded_storage_async;

pub enum DummyFlash {}

impl ekv::flash::Flash for DummyFlash {
    type Error = ();

    fn page_count(&self) -> usize {
        unimplemented!()
    }

    async fn erase(&mut self, _page_id: ekv::flash::PageID) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn read(
        &mut self,
        _page_id: ekv::flash::PageID,
        _offset: usize,
        _data: &mut [u8],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn write(
        &mut self,
        _page_id: ekv::flash::PageID,
        _offset: usize,
        _data: &[u8],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

pub type DummyStorage = Database<DummyFlash, CriticalSectionRawMutex>;
