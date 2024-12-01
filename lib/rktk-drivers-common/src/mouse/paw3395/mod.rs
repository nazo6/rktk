#![allow(dead_code)]

pub mod config;
mod error;
mod power_up;
mod registers;

use embassy_time::Timer;
use embedded_hal::spi::Operation;
use embedded_hal_async::spi::SpiDevice;
use error::Paw3395Error;
use registers as reg;
use rktk::drivers::interface::{mouse::MouseDriver, DriverBuilder};

mod timing {
    pub const NCS_SCLK: u32 = 120;
    pub const SCLK_NCS_READ: u32 = 120;
    pub const SCLK_NCS_WRITE: u32 = 1000;
    pub const SRAD: u32 = 2 * 1000;
    pub const SWW_R: u32 = 5 * 1000;
    pub const SRW_R: u32 = 2 * 1000;
    pub const BEXIT: u32 = 500;
}

#[derive(Default, Debug)]
pub struct BurstData {
    pub op_mode: u8,
    pub lift_stat: bool,
    pub mot: bool,
    pub observation: u8,
    pub dx: i16,
    pub dy: i16,
    pub surface_quality: u8,
    pub raw_data_sum: u8,
    pub max_raw_data: u8,
    pub min_raw_data: u8,
    pub shutter: u16,
}

pub struct Paw3395Builder<S: SpiDevice> {
    spi: S,
    config: config::Config,
}

impl<S: SpiDevice> Paw3395Builder<S> {
    pub fn new(spi: S, config: config::Config) -> Self {
        Self { spi, config }
    }
}

impl<S: SpiDevice> DriverBuilder for Paw3395Builder<S> {
    type Output = Paw3395<S>;

    type Error = Paw3395Error<S::Error>;

    async fn build(self) -> Result<Self::Output, Self::Error> {
        let mut driver = Paw3395 {
            spi: self.spi,
            timer: embassy_time::Instant::now(),
            config: self.config,
        };

        driver.power_up().await?;

        Ok(driver)
    }
}

pub struct Paw3395<S: SpiDevice> {
    spi: S,
    timer: embassy_time::Instant,
    config: config::Config,
}

impl<S: SpiDevice> MouseDriver for Paw3395<S> {
    type Error = Paw3395Error<S::Error>;

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
        Err(Paw3395Error::NotSupported)
    }
}

impl<S: SpiDevice> Paw3395<S> {
    async fn write(&mut self, address: u8, data: u8) -> Result<(), Paw3395Error<S::Error>> {
        self.spi
            .transaction(&mut [
                Operation::DelayNs(timing::NCS_SCLK),
                Operation::Write(&[address | 0x80, data]),
                Operation::DelayNs(timing::SCLK_NCS_WRITE),
            ])
            .await
            .map_err(Paw3395Error::Spi)?;

        Timer::after_nanos((timing::SWW_R - timing::SCLK_NCS_WRITE) as u64).await;

        Ok(())
    }

    async fn read(&mut self, address: u8) -> Result<u8, Paw3395Error<S::Error>> {
        let mut buf = [0x00];
        self.spi
            .transaction(&mut [
                Operation::DelayNs(timing::NCS_SCLK),
                // send adress of the register, with MSBit = 0 to indicate it's a read
                Operation::Write(&[address & 0x7f]),
                Operation::DelayNs(timing::SRAD),
                Operation::Read(&mut buf),
                Operation::DelayNs(timing::SCLK_NCS_READ),
            ])
            .await
            .map_err(Paw3395Error::Spi)?;

        //  tSRW/tSRR minus tSCLK-NCS
        Timer::after_nanos((timing::SRW_R - timing::SCLK_NCS_WRITE) as u64).await;

        Ok(buf[0])
    }

    pub async fn burst_read(&mut self) -> Result<BurstData, Paw3395Error<S::Error>> {
        let mut buf = [0u8; 12];
        self.spi
            .transaction(&mut [
                Operation::DelayNs(timing::NCS_SCLK),
                Operation::Write(&[reg::MOTION_BURST]),
                Operation::DelayNs(timing::SRAD),
                Operation::Read(&mut buf),
            ])
            .await
            .map_err(Paw3395Error::Spi)?;

        Timer::after_nanos(timing::BEXIT as u64).await;

        //combine the register values
        let data = BurstData {
            op_mode: buf[0] & 0b11,
            lift_stat: buf[0] >> 3 & 1 == 1,
            mot: buf[0] >> 7 & 1 == 1,
            observation: buf[1],
            dx: (buf[3] as i16) << 8 | (buf[2] as i16),
            dy: (buf[5] as i16) << 8 | (buf[4] as i16),
            surface_quality: buf[6],
            raw_data_sum: buf[7],
            max_raw_data: buf[8],
            min_raw_data: buf[9],
            shutter: (buf[11] as u16) << 8 | (buf[10] as u16),
        };

        Ok(data)
    }

    pub async fn set_cpi(&mut self, cpi: u16) -> Result<(), Paw3395Error<S::Error>> {
        let resolution = (cpi / 50) - 1;
        let resolution_low = resolution as u8;
        let resolution_high = (resolution >> 8) as u8;

        self.write(reg::RESOLUTION_X_LOW, resolution_low).await?;
        self.write(reg::RESOLUTION_X_HIGH, resolution_high).await?;
        self.write(reg::RESOLUTION_Y_LOW, resolution_low).await?;
        self.write(reg::RESOLUTION_Y_HIGH, resolution_high).await?;
        self.write(reg::SET_RESOLUTION, 0x01).await?;

        Ok(())
    }

    pub async fn get_cpi(&mut self) -> Result<u16, S::Error> {
        let resolution_x_low = self.read(reg::RESOLUTION_X_LOW).await.unwrap_or_default();
        let resolution_x_high = self.read(reg::RESOLUTION_X_HIGH).await.unwrap_or_default();
        let resolution_x = ((resolution_x_high as u16) << 8) | resolution_x_low as u16;
        Ok((resolution_x + 1) * 50)
    }

    pub async fn check_signature(&mut self) -> Result<bool, Paw3395Error<S::Error>> {
        let pid = self.read(reg::PRODUCT_ID).await.unwrap_or(0);
        let ipid = self.read(reg::INV_PRODUCT_ID).await.unwrap_or(0);

        Ok(pid == 0x51 && ipid == 0xAE)
    }

    async fn shutdown(&mut self) -> Result<(), Paw3395Error<S::Error>> {
        self.write(reg::SHUTDOWN, 0xB6).await?;
        Timer::after_millis(5).await;
        Ok(())
    }

    async fn power_up(&mut self) -> Result<(), Paw3395Error<S::Error>> {
        Timer::after_micros(50).await;

        self.write(reg::POWER_UP_RESET, 0x5A).await?;
        Timer::after_millis(5).await;

        for (addr, data) in power_up::POWER_UP_SEQS_1.iter() {
            self.write(*addr, *data).await?;
        }

        'outer: {
            for _ in 0..60 {
                Timer::after_millis(1).await;
                if self.read(0x6C).await? == 0x80 {
                    break 'outer;
                };
            }

            for (addr, data) in power_up::POWER_UP_SEQS_2.iter() {
                self.write(*addr, *data).await?;
            }
        }

        for (addr, data) in power_up::POWER_UP_SEQS_3.iter() {
            self.write(*addr, *data).await?;
        }

        self.read(reg::MOTION).await?;
        self.read(reg::DELTA_X_L).await?;
        self.read(reg::DELTA_X_H).await?;
        self.read(reg::DELTA_Y_L).await?;
        self.read(reg::DELTA_Y_H).await?;

        if !self.check_signature().await.unwrap_or(false) {
            return Err(Paw3395Error::InvalidSignature);
        }

        Timer::after_micros(100).await;

        // set mode
        for (addr, data) in self.config.mode.commands.iter() {
            self.write(*addr, *data).await?;
        }
        if let Some(c) = self.config.mode._0x40 {
            let mut _0x40 = self.read(0x40).await?;
            _0x40 |= c;
            self.write(0x40, _0x40).await?;
        }

        // set lift cutoff config
        self.write(0x7F, 0x0C).await?;
        let lift_config = self.read(0x4E).await?;
        self.write(0x7F, 0x00).await?;
        let lift_config = lift_config | self.config.lift_cutoff as u8;
        self.write(0x7F, 0x0C).await?;
        self.write(0x4E, lift_config).await?;
        self.write(0x7F, 0x00).await?;

        Ok(())
    }
}
