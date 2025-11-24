//! PMW3360 optical sensor driver.

#![allow(dead_code)]

mod error;
mod registers;
mod srom_liftoff;
mod srom_tracking;

use embassy_time::Timer;
use embedded_hal_async::spi::Operation;

use error::Pmw3360Error;
use registers as reg;
use rktk::drivers::interface::mouse::MouseDriver;

use crate::spi::{ExtendedSpi, IterOperation};

mod timing {
    /// NCS To SCLK Active
    pub const NCS_SCLK: u32 = 120;
}

#[derive(Default, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BurstData {
    pub motion: bool,
    pub on_surface: bool,
    pub op_mode: u8,
    pub frame_data_first: bool,
    pub dx: i16,
    pub dy: i16,
    pub surface_quality: u8,
    pub raw_data_sum: u8,
    pub max_raw_data: u8,
    pub min_raw_data: u8,
    pub shutter: u16,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmw3360Srom {
    None,
    Liftoff,
    #[default]
    Tracking,
}

impl Pmw3360Srom {
    fn id(&self) -> u8 {
        match self {
            Pmw3360Srom::None => 0x00,
            Pmw3360Srom::Liftoff => 0x81,
            Pmw3360Srom::Tracking => 0x04,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Pmw3360Config {
    pub srom: Pmw3360Srom,
    pub cpi: u16,
    pub auto_reset: bool,
}

impl Default for Pmw3360Config {
    fn default() -> Self {
        Self {
            srom: Pmw3360Srom::default(),
            cpi: 1000,
            auto_reset: false,
        }
    }
}

/// PMW3360 optical sensor driver.
///
/// # Notice about SPI
/// This driver requires [`ExtendedSpi`] trait for SPI communication instead of
/// [`embedded_hal_async::spi::SpiDevice`].
/// This is because the PMW3360 driver needs to perform streaming transfers with CS held low,
///
/// Although, you still can use any SPI device that implements
/// [`embedded_hal_async::spi::SpiDevice`]. In that case, SROM download will be skipped.
pub struct Pmw3360<S: ExtendedSpi> {
    spi_device: S,
    in_burst_mode: bool,
    config: Pmw3360Config,
}

impl<S: ExtendedSpi> Pmw3360<S> {
    pub fn new(spi_device: S, config: Pmw3360Config) -> Self {
        Self {
            spi_device,
            in_burst_mode: false,
            config,
        }
    }
}

impl<S: ExtendedSpi> MouseDriver for Pmw3360<S> {
    type Error = Pmw3360Error<S::Error>;

    async fn init(&mut self) -> Result<(), Self::Error> {
        self.power_up().await
    }

    async fn read(&mut self) -> Result<(i8, i8), Self::Error> {
        self.burst_read()
            .await
            .map(|data| (data.dx as i8, data.dy as i8))
    }

    async fn set_cpi(&mut self, cpi: u16) -> Result<(), Self::Error> {
        self.set_cpi(cpi).await?;
        Ok(())
    }

    async fn get_cpi(&mut self) -> Result<u16, Self::Error> {
        Err(Self::Error::NotSupported)
    }
}

impl<S: ExtendedSpi> Pmw3360<S> {
    async fn write(&mut self, address: u8, data: u8) -> Result<(), <Self as MouseDriver>::Error> {
        self.in_burst_mode = false;
        self.spi_device
            .transaction(&mut [
                Operation::DelayNs(timing::NCS_SCLK),
                // send adress of the register, with MSBit = 1 to indicate it's a write and send data
                Operation::TransferInPlace(&mut [address | 0x80, data]),
                // tSCLK-NCS (write)
                Operation::DelayNs(35 * 1000),
            ])
            .await
            .map_err(Pmw3360Error::Spi)?;

        // tSWW/tSWR minus tSCLK-NCS (write)
        Timer::after_micros(145).await;

        Ok(())
    }

    async fn read(&mut self, address: u8) -> Result<u8, <Self as MouseDriver>::Error> {
        self.in_burst_mode = false;
        let mut buf = [0x00];
        self.spi_device
            .transaction(&mut [
                Operation::DelayNs(timing::NCS_SCLK),
                // send adress of the register, with MSBit = 0 to indicate it's a read
                Operation::Write(&[address & 0x7f]),
                // tSRAD
                Operation::DelayNs(160 * 1000),
                // read the data
                Operation::Read(&mut buf),
                // tSCLK-NCS (read)
                Operation::DelayNs(120),
            ])
            .await
            .map_err(Pmw3360Error::Spi)?;

        //  tSRW/tSRR
        Timer::after_micros(20).await;

        Ok(buf[0])
    }

    pub async fn burst_read(&mut self) -> Result<BurstData, <Self as MouseDriver>::Error> {
        if !self.in_burst_mode {
            self.write(reg::MOTION_BURST, 0x00).await?;
            self.in_burst_mode = true;
        }

        let mut data = [0u8; 12];

        self.spi_device
            .transaction(&mut [
                Operation::DelayNs(timing::NCS_SCLK),
                Operation::Write(&[reg::MOTION_BURST]),
                // tSRAD-MOTBR
                Operation::DelayNs(35 * 1000),
                Operation::Read(&mut data),
            ])
            .await
            .map_err(Pmw3360Error::Spi)?;

        // tBEXIT
        Timer::after_micros(1).await;

        //combine the register values
        let data = BurstData {
            motion: (data[0] & 0x80) != 0,
            on_surface: (data[0] & 0x08) == 0,
            op_mode: (data[0] >> 1) & 0x03,
            frame_data_first: data[0] & 0x01 != 0,
            dx: ((data[3] as i16) << 8) | (data[2] as i16),
            dy: ((data[5] as i16) << 8) | (data[4] as i16),
            surface_quality: data[6],
            raw_data_sum: data[7],
            max_raw_data: data[8],
            min_raw_data: data[9],
            shutter: ((data[11] as u16) << 8) | (data[10] as u16),
        };

        // FIXME: This is a workaround for a phenomenon in which the PMW3360 sensor stacks with
        // OP_MODE remaining at Rest1 and then stops moving.
        // It is necessary to investigate whether this is hardware-specific or whether other programs are incorrect.
        if data.motion
            && data.op_mode == 0x01
            && data.on_surface
            && data.dx == 0
            && data.dy == 0
            && self.config.auto_reset
        {
            rktk_log::warn!("Invalid motion detected. Performing reset:\n{:?}", data);
            self.power_up().await?;
        }

        Ok(data)
    }

    pub async fn set_cpi(&mut self, cpi: u16) -> Result<(), <Self as MouseDriver>::Error> {
        self.config.cpi = cpi;
        let val: u16;
        if cpi < 100 {
            val = 0
        } else if cpi > 12000 {
            val = 0x77
        } else {
            val = (cpi - 100) / 100;
        }
        self.write(reg::CONFIG_1, val as u8).await?;
        Ok(())
    }

    pub async fn get_cpi(&mut self) -> Result<u16, <Self as MouseDriver>::Error> {
        let val = self.read(reg::CONFIG_1).await.unwrap_or_default() as u16;
        Ok((val + 1) * 100)
    }

    pub async fn check_signature(&mut self) -> Result<bool, <Self as MouseDriver>::Error> {
        let srom = self.read(reg::SROM_ID).await.unwrap_or(0);
        let pid = self.read(reg::PRODUCT_ID).await.unwrap_or(0);
        let ipid = self.read(reg::INVERSE_PRODUCT_ID).await.unwrap_or(0);

        Ok(srom == self.config.srom.id() && pid == 0x42 && ipid == 0xBD)
    }

    #[allow(dead_code)]
    pub async fn self_test(&mut self) -> Result<bool, <Self as MouseDriver>::Error> {
        self.write(reg::SROM_ENABLE, 0x15).await?;
        Timer::after_micros(10000).await;

        let u = self.read(reg::DATA_OUT_UPPER).await.unwrap_or(0); // should be 0xBE
        let l = self.read(reg::DATA_OUT_LOWER).await.unwrap_or(0); // should be 0xEF

        Ok(u == 0xBE && l == 0xEF)
    }

    async fn power_up(&mut self) -> Result<(), <Self as MouseDriver>::Error> {
        let is_valid_signature = self.power_up_inner().await?;
        if is_valid_signature {
            self.set_cpi(self.config.cpi).await?;
            Ok(())
        } else {
            Err(Pmw3360Error::InvalidSignature)
        }
    }

    async fn power_up_inner(&mut self) -> Result<bool, <Self as MouseDriver>::Error> {
        // reset spi port
        self.spi_device
            .transaction(&mut [])
            .await
            .map_err(Pmw3360Error::Spi)?;

        // Write to reset register
        self.write(reg::POWER_UP_RESET, 0x5A).await?;

        // Wait at least 50ms
        Timer::after_millis(100).await;

        // read registers 0x02 to 0x06
        self.read(reg::MOTION).await?;
        self.read(reg::DELTA_X_L).await?;
        self.read(reg::DELTA_X_H).await?;
        self.read(reg::DELTA_Y_L).await?;
        self.read(reg::DELTA_Y_H).await?;

        // perform SROM download
        self.srom_download().await?;

        let is_valid_signature = self.check_signature().await.unwrap_or(false);

        // Write 0x00 (rest disable) to Config2 register for wired mouse or 0x20 for
        // wireless mouse design.
        self.write(reg::CONFIG_2, 0x00).await?;

        Timer::after_micros(100).await;

        Ok(is_valid_signature)
    }

    async fn srom_download(&mut self) -> Result<(), <Self as MouseDriver>::Error> {
        let fw = match self.config.srom {
            Pmw3360Srom::None => {
                rktk_log::info!("SROM download skipped: no SROM selected");
                return Ok(());
            }
            Pmw3360Srom::Liftoff => &srom_liftoff::FW,
            Pmw3360Srom::Tracking => &srom_tracking::FW,
        };
        if !self.spi_device.transaction_iter_supported() {
            rktk_log::warn!("SROM download skipped: transaction_iter not supported");
            return Ok(());
        }

        // Write 0 to Rest_En bit of Config2 register to disable Rest mode.
        self.write(reg::CONFIG_2, 0x00).await?;

        // write 0x1d in SROM_enable reg for initializing
        self.write(reg::SROM_ENABLE, 0x1d).await?;

        // wait for 10 ms
        Timer::after_micros(10000).await;

        // Write 0x18 to SROM_Enable register again to start SROM Download
        self.write(reg::SROM_ENABLE, 0x18).await?;

        let operations = [IterOperation::Write(reg::SROM_LOAD_BURST | 0x80)]
            .into_iter()
            .chain(fw.iter().flat_map(|byte| {
                [
                    IterOperation::Write(*byte),
                    IterOperation::DelayNs(15 * 1000),
                ]
            }));

        self.spi_device
            .transaction_iter(operations)
            .await
            .map_err(Pmw3360Error::Spi)?;

        Timer::after_micros(185).await;

        let srom_id = self.read(reg::SROM_ID).await?;
        if srom_id != self.config.srom.id() {
            rktk_log::error!("SROM Download failed. ID: 0x{:02x}", srom_id);
            return Err(Pmw3360Error::InvalidSignature);
        }
        rktk_log::debug!("SROM Download succeeded. ID: 0x{:02x}", srom_id);

        Ok(())
    }
}
