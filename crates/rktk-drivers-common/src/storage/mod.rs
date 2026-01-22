//! Storage driver implementations.

pub mod flash_sequential_map;

pub mod flash_region {
    use core::ops::Range;
    use embedded_storage_async::nor_flash::{
        ErrorType, MultiwriteNorFlash, NorFlash, ReadNorFlash,
    };
    use rktk::utils::Mutex;

    #[derive(Debug, Clone)]
    pub enum FlashSplitError {
        OverlappingRegions,
        TooLargeRegions,
    }

    pub struct FlashRegion<'a, F: NorFlash> {
        flash: &'a Mutex<F>,
        region: Range<u32>,
    }

    impl<'a, F: NorFlash> FlashRegion<'a, F> {
        pub fn new(flash: &'a Mutex<F>, region: Range<u32>) -> Self {
            Self { flash, region }
        }

        pub fn split(
            self,
            region_first: Range<u32>,
            region_second: Range<u32>,
        ) -> Result<(Self, Self), FlashSplitError> {
            if region_first.start < region_second.end && region_second.start < region_first.end {
                return Err(FlashSplitError::OverlappingRegions);
            }
            if region_first.end.max(region_second.end) > self.region.end
                || region_first.start.min(region_second.start) < self.region.start
            {
                return Err(FlashSplitError::TooLargeRegions);
            }

            Ok((
                FlashRegion::new(self.flash, region_first),
                FlashRegion::new(self.flash, region_second),
            ))
        }
    }

    impl<'a, F: NorFlash> ErrorType for FlashRegion<'a, F> {
        type Error = F::Error;
    }

    impl<'a, F: NorFlash> ReadNorFlash for FlashRegion<'a, F> {
        const READ_SIZE: usize = F::READ_SIZE;

        async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
            let mut flash = self.flash.lock().await;
            flash.read(self.region.start + offset, bytes).await
        }

        fn capacity(&self) -> usize {
            self.region.end as usize - self.region.start as usize
        }
    }
    impl<'a, F: NorFlash> NorFlash for FlashRegion<'a, F> {
        const WRITE_SIZE: usize = F::WRITE_SIZE;

        const ERASE_SIZE: usize = F::ERASE_SIZE;

        async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
            let mut flash = self.flash.lock().await;
            flash
                .erase(self.region.start + from, self.region.start + to)
                .await
        }

        async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
            let mut flash = self.flash.lock().await;
            flash.write(self.region.start + offset, bytes).await
        }
    }
    impl<'a, F: MultiwriteNorFlash> MultiwriteNorFlash for FlashRegion<'a, F> {}
}
