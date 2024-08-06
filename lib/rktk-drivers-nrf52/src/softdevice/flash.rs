use ekv::Database;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embedded_storage_async::nor_flash::{NorFlash as _, ReadNorFlash};
use nrf_softdevice::Softdevice;
use static_cell::StaticCell;

pub struct FlashWrapper(nrf_softdevice::Flash);

pub type NrfDb = Database<FlashWrapper, CriticalSectionRawMutex>;

const START: u32 = 0x00081920;

static DATABASE: StaticCell<NrfDb> = StaticCell::new();

/// Get steal from softdevice instance.
///
/// This function must be called only once. Otherwise, it will panic.
pub fn init_database(sd: &Softdevice, random: u32) -> &'static mut NrfDb {
    let mut config = ekv::Config::default();
    config.random_seed = random;
    DATABASE.init(ekv::Database::new(
        FlashWrapper(nrf_softdevice::Flash::take(sd)),
        config,
    ))
}

impl ekv::flash::Flash for FlashWrapper {
    type Error = nrf_softdevice::FlashError;

    fn page_count(&self) -> usize {
        ekv::config::MAX_PAGE_COUNT
    }

    async fn erase(&mut self, page_id: ekv::flash::PageID) -> Result<(), Self::Error> {
        let start = START + page_id.index() as u32 * ekv::config::PAGE_SIZE as u32;
        let end = start + ekv::config::PAGE_SIZE as u32;
        self.0.erase(start, end).await
    }

    async fn read(
        &mut self,
        page_id: ekv::flash::PageID,
        offset: usize,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.0
            .read(
                page_id.index() as u32 * ekv::config::PAGE_SIZE as u32 + offset as u32,
                data,
            )
            .await
    }

    async fn write(
        &mut self,
        page_id: ekv::flash::PageID,
        offset: usize,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        self.0
            .write(
                page_id.index() as u32 * ekv::config::PAGE_SIZE as u32 + offset as u32,
                data,
            )
            .await
    }
}
