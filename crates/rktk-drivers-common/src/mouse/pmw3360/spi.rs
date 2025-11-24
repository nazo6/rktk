use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex};
use embedded_hal::{
    digital::OutputPin,
    spi::{ErrorType, Operation},
};
use embedded_hal_async::spi::{SpiBus, SpiDevice as SpiDeviceTrait};

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IterOperation<Word: 'static> {
    Write(Word),
    DelayNs(u32),
}

pub trait ExtendedSpi<Word: Copy + 'static = u8> {
    type Error: core::fmt::Debug;

    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), Self::Error>;

    fn transaction_iter_supported(&self) -> bool {
        false
    }

    async fn transaction_iter(
        &mut self,
        _operations: impl Iterator<Item = IterOperation<Word>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct EmbassySpiDevice<
    'a,
    M: RawMutex,
    BUS: SpiBus<Word>,
    CS: OutputPin,
    Word: Copy + 'static = u8,
> {
    bus: &'a Mutex<M, BUS>,
    cs: CS,
    _word: core::marker::PhantomData<Word>,
}

impl<'a, M: RawMutex, BUS: SpiBus<Word>, CS: OutputPin, Word: Copy + 'static>
    EmbassySpiDevice<'a, M, BUS, CS, Word>
{
    pub fn new(bus: &'a Mutex<M, BUS>, cs: CS) -> Self {
        Self {
            bus,
            cs,
            _word: core::marker::PhantomData,
        }
    }
}

impl<'a, M: RawMutex + 'a, BUS: SpiBus<Word>, CS: OutputPin + 'a, Word: Copy + 'static>
    ExtendedSpi<Word> for EmbassySpiDevice<'a, M, BUS, CS, Word>
{
    type Error = <SpiDevice<'a, M, BUS, CS> as ErrorType>::Error;

    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), Self::Error> {
        let mut spi = SpiDevice::new(self.bus, &mut self.cs);
        SpiDeviceTrait::transaction(&mut spi, operations).await
    }

    fn transaction_iter_supported(&self) -> bool {
        true
    }

    async fn transaction_iter(
        &mut self,
        operations: impl Iterator<Item = IterOperation<Word>>,
    ) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Self::Error::Cs)?;
        {
            let mut bus = self.bus.lock().await;
            for op in operations {
                match op {
                    IterOperation::Write(item) => {
                        bus.write(&[item]).await.map_err(Self::Error::Spi)?
                    }
                    IterOperation::DelayNs(n) => embassy_time::Timer::after_nanos(n.into()).await,
                }
            }
        }
        self.cs.set_high().map_err(Self::Error::Cs)?;

        Ok(())
    }
}

impl<T: SpiDeviceTrait<Word>, Word: Copy + 'static> ExtendedSpi<Word> for T {
    type Error = T::Error;

    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, Word>],
    ) -> Result<(), Self::Error> {
        SpiDeviceTrait::transaction(self, operations).await
    }
}
