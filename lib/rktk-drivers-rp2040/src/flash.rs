use ekv::config;
use embassy_rp::{
    clocks::RoscRng,
    dma::Channel,
    flash::{Async, Error, Flash, Instance},
    Peripheral,
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use rand_core::RngCore as _;
use rktk::interface::storage::{
    ekv::{
        self,
        flash::{Flash as EkvFlash, PageID},
    },
    embedded_storage_async::nor_flash::NorFlash,
};

pub fn init_db<'a, T: Instance, const SIZE: usize>(
    flash: impl Peripheral<P = T> + 'a,
    dma: impl Peripheral<P = impl Channel> + 'a,
) -> ekv::Database<RpFlash<'a, T, SIZE>, CriticalSectionRawMutex> {
    let flash = RpFlash::new(flash, dma);
    let mut config = ekv::Config::default();
    config.random_seed = RoscRng.next_u32();
    ekv::Database::new(flash, config)
}

// assume the firmware size is 1MB
const FLASH_START: usize = 1024 * 1024;

pub struct RpFlash<'a, T: Instance, const SIZE: usize> {
    flash: Flash<'a, T, Async, SIZE>,
    start: usize,
}

impl<'a, T: Instance, const SIZE: usize> RpFlash<'a, T, SIZE> {
    pub fn new(
        flash: impl Peripheral<P = T> + 'a,
        dma: impl Peripheral<P = impl Channel> + 'a,
    ) -> Self {
        Self {
            flash: Flash::new(flash, dma),
            start: FLASH_START,
        }
    }
}

// Workaround for alignment requirements.
#[repr(C, align(4))]
struct AlignedBuf<const N: usize>([u8; N]);

impl<'a, T: Instance, const SIZE: usize> EkvFlash for RpFlash<'a, T, SIZE> {
    type Error = Error;

    fn page_count(&self) -> usize {
        (self.flash.capacity() - self.start) / config::PAGE_SIZE
    }

    async fn erase(&mut self, page_id: PageID) -> Result<(), Self::Error> {
        self.flash
            .erase(
                (self.start + page_id.index() * config::PAGE_SIZE) as u32,
                (self.start + page_id.index() * config::PAGE_SIZE + config::PAGE_SIZE) as u32,
            )
            .await
    }

    async fn read(
        &mut self,
        page_id: PageID,
        offset: usize,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        let address = self.start + page_id.index() * config::PAGE_SIZE + offset;
        let mut buf = AlignedBuf([0; config::PAGE_SIZE]);
        self.flash
            .read(address as u32, &mut buf.0[..data.len()])
            .await?;
        data.copy_from_slice(&buf.0[..data.len()]);
        Ok(())
    }

    async fn write(
        &mut self,
        page_id: PageID,
        offset: usize,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let address = self.start + page_id.index() * config::PAGE_SIZE + offset;
        let mut buf = AlignedBuf([0; config::PAGE_SIZE]);
        buf.0[..data.len()].copy_from_slice(data);
        self.flash.write(address as u32, &buf.0[..data.len()]).await
    }
}
