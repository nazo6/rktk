//! PMW3360 optical sensor driver.

#![allow(dead_code)]

mod error;
mod registers;
mod srom_liftoff;
mod srom_tracking;

use embassy_embedded_hal::shared_bus::{SpiDeviceError, asynch::spi::SpiDevice};
use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex};
use embassy_time::Timer;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::{Operation, SpiBus, SpiDevice as _};

use error::Pmw3360Error;
use registers as reg;
use rktk::drivers::interface::mouse::MouseDriver;

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

pub struct Pmw3360<'a, M: RawMutex, BUS: SpiBus<u8>, CS: OutputPin> {
    bus: &'a Mutex<M, BUS>,
    cs: CS,
    in_burst_mode: bool,
    cpi: u16,
}

impl<'a, M: RawMutex, BUS: SpiBus, CS: OutputPin> Pmw3360<'a, M, BUS, CS> {
    pub fn new(bus: &'a Mutex<M, BUS>, cs: CS) -> Self {
        Self {
            bus,
            cs,
            in_burst_mode: false,
            cpi: 1000, // Default CPI
        }
    }

    fn spi(&mut self) -> SpiDevice<'a, M, BUS, &mut CS> {
        SpiDevice::new(self.bus, &mut self.cs)
    }
}

impl<'a, M: RawMutex, BUS: SpiBus, CS: OutputPin> MouseDriver for Pmw3360<'a, M, BUS, CS> {
    type Error = Pmw3360Error<SpiDeviceError<BUS::Error, CS::Error>>;

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

impl<'a, M: RawMutex, BUS: SpiBus, CS: OutputPin> Pmw3360<'a, M, BUS, CS> {
    async fn write(&mut self, address: u8, data: u8) -> Result<(), <Self as MouseDriver>::Error> {
        self.in_burst_mode = false;
        self.spi()
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
        self.spi()
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

        self.spi()
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

        // // FIXME: This is a workaround for a phenomenon in which the PMW3360 sensor stacks with
        // // OP_MODE remaining at Rest1 and then stops moving.
        // // It is necessary to investigate whether this is hardware-specific or whether other programs are incorrect.
        // if data.motion && data.op_mode == 0x01 && data.on_surface && data.dx == 0 && data.dy == 0 {
        //     rktk_log::warn!("Invalid motion detected. Performing reset:\n{:?}", data);
        //     self.power_up().await?;
        // }

        Ok(data)
    }

    pub async fn set_cpi(&mut self, cpi: u16) -> Result<(), <Self as MouseDriver>::Error> {
        self.cpi = cpi;
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

        // signature for SROM 0x04
        Ok(srom == 0x04 && pid == 0x42 && ipid == 0xBD)
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
            self.set_cpi(self.cpi).await?;
            Ok(())
        } else {
            Err(Pmw3360Error::InvalidSignature)
        }
    }

    async fn power_up_inner(&mut self) -> Result<bool, <Self as MouseDriver>::Error> {
        // reset spi port
        self.spi()
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
        // Write 0 to Rest_En bit of Config2 register to disable Rest mode.
        self.write(reg::CONFIG_2, 0x00).await?;

        // write 0x1d in SROM_enable reg for initializing
        self.write(reg::SROM_ENABLE, 0x1d).await?;

        // wait for 10 ms
        Timer::after_micros(10000).await;

        // Write 0x18 to SROM_Enable register again to start SROM Download
        self.write(reg::SROM_ENABLE, 0x18).await?;

        self.cs
            .set_low()
            .map_err(|e| Pmw3360Error::Spi(SpiDeviceError::Cs(e)))?;

        let mut spi_err = None;
        {
            let mut bus = self.bus.lock().await;
            let _ = bus.write(&[reg::SROM_LOAD_BURST | 0x80]).await;
            for byte in srom_tracking::FW {
                let res = bus.write(&[byte]).await;
                Timer::after_micros(15).await;
                if let Err(e) = res {
                    spi_err = Some(e);
                    break;
                }
            }
        }
        self.cs
            .set_high()
            .map_err(|e| Pmw3360Error::Spi(SpiDeviceError::Cs(e)))?;

        Timer::after_micros(185).await;

        if let Some(e) = spi_err {
            return Err(Pmw3360Error::Spi(SpiDeviceError::Spi(e)));
        }

        let srom_id = self.read(reg::SROM_ID).await?;
        if srom_id == 0x00 {
            rktk_log::error!("SROM Download failed. ID: 0x{:02x}", srom_id);
            return Err(Pmw3360Error::InvalidSignature);
        }

        Ok(())
    }
}
