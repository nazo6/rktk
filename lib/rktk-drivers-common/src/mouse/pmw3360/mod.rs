#![allow(dead_code)]

mod registers;
mod srom_liftoff;
mod srom_tracking;

use embassy_time::Timer;
use embedded_hal::{digital::OutputPin, spi::ErrorType};
use embedded_hal_async::spi::SpiBus;
use registers as reg;
use rktk::interface::{mouse::Mouse, DriverBuilder};

#[derive(Default)]
pub struct BurstData {
    pub motion: bool,
    pub on_surface: bool,
    pub dx: i16,
    pub dy: i16,
    pub surface_quality: u8,
    pub raw_data_sum: u8,
    pub max_raw_data: u8,
    pub min_raw_data: u8,
    pub shutter: u16,
}

#[derive(Debug)]
pub enum Pmw3360Error<SE: ErrorType> {
    InvalidSignature,
    Spi(SE::Error),
}

pub struct Pmw3360<'d, S: SpiBus + 'd, OP: OutputPin + 'd> {
    _marker: core::marker::PhantomData<&'d ()>,
    spi: S,
    cs_pin: OP,
    // reset_pin: RESET,
    // rw_flag is set if any writes or reads were performed
    rw_flag: bool,
}

impl<'d, S: SpiBus + 'd, OP: OutputPin + 'd> Pmw3360<'d, S, OP> {
    pub fn new(spi: S, cs_pin: OP) -> Self {
        Self {
            _marker: core::marker::PhantomData,
            spi,
            cs_pin,
            rw_flag: false,
        }
    }

    pub async fn burst_read(&mut self) -> Result<BurstData, S::Error> {
        // TODO: propagate errors

        // Write any value to Motion_burst register
        // if any write occured before
        if self.rw_flag {
            self.write(reg::MOTION_BURST, 0x00).await?;
            self.rw_flag = false;
        }

        // Lower NCS
        self.cs_pin.set_low();
        // Send Motion_burst address
        self.spi.transfer_in_place(&mut [reg::MOTION_BURST]).await?;

        // tSRAD_MOTBR
        Timer::after_micros(35).await;

        // Read the 12 bytes of burst data
        let mut buf = [0u8; 12];
        for b in buf.iter_mut() {
            let t_buf = &mut [0x00];
            match self.spi.transfer_in_place(t_buf).await {
                Ok(()) => *b = *t_buf.first().unwrap(),
                Err(_) => *b = 0,
            }
        }

        // Raise NCS
        self.cs_pin.set_high();
        // tBEXIT
        Timer::after_micros(1).await;

        //combine the register values
        let data = BurstData {
            motion: (buf[0] & 0x80) != 0,
            on_surface: (buf[0] & 0x08) == 0, // 0 if on surface / 1 if off surface
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

    pub async fn set_cpi(&mut self, cpi: u16) -> Result<(), S::Error> {
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

    pub async fn get_cpi(&mut self) -> Result<u16, S::Error> {
        let val = self.read(reg::CONFIG_1).await.unwrap_or_default() as u16;
        Ok((val + 1) * 100)
    }

    pub async fn check_signature(&mut self) -> Result<bool, S::Error> {
        // TODO: propagate errors

        let srom = self.read(reg::SROM_ID).await.unwrap_or(0);
        let pid = self.read(reg::PRODUCT_ID).await.unwrap_or(0);
        let ipid = self.read(reg::INVERSE_PRODUCT_ID).await.unwrap_or(0);

        // signature for SROM 0x04
        Ok(srom == 0x04 && pid == 0x42 && ipid == 0xBD)
    }

    #[allow(dead_code)]
    pub async fn self_test(&mut self) -> Result<bool, S::Error> {
        self.write(reg::SROM_ENABLE, 0x15).await?;
        Timer::after_micros(10000).await;

        let u = self.read(reg::DATA_OUT_UPPER).await.unwrap_or(0); // should be 0xBE
        let l = self.read(reg::DATA_OUT_LOWER).await.unwrap_or(0); // should be 0xEF

        Ok(u == 0xBE && l == 0xEF)
    }

    async fn write(&mut self, address: u8, data: u8) -> Result<(), S::Error> {
        // TODO: propagate errors

        self.cs_pin.set_low();
        // tNCS-SCLK
        Timer::after_micros(1).await;

        // send adress of the register, with MSBit = 1 to indicate it's a write
        self.spi.transfer_in_place(&mut [address | 0x80]).await?;
        // send data
        self.spi.transfer_in_place(&mut [data]).await?;

        // tSCLK-NCS (write)
        Timer::after_micros(35).await;
        self.cs_pin.set_high();

        // tSWW/tSWR minus tSCLK-NCS (write)
        Timer::after_micros(145).await;

        self.rw_flag = true;

        Ok(())
    }

    async fn read(&mut self, address: u8) -> Result<u8, S::Error> {
        // TODO: propagate errors
        self.cs_pin.set_low();
        // tNCS-SCLK
        Timer::after_micros(1).await;

        // send adress of the register, with MSBit = 0 to indicate it's a read
        self.spi.transfer_in_place(&mut [address & 0x7f]).await?;

        // tSRAD
        Timer::after_micros(160).await;

        let mut ret = 0;
        let mut buf = [0x00];
        if (self.spi.transfer_in_place(&mut buf).await).is_ok() {
            ret = *buf.first().unwrap();
        }

        // tSCLK-NCS (read)
        Timer::after_micros(1).await;
        self.cs_pin.set_high();

        //  tSRW/tSRR minus tSCLK-NCS
        Timer::after_micros(20).await;

        self.rw_flag = true;

        Ok(ret)
    }

    async fn power_up_inner(&mut self) -> Result<bool, S::Error> {
        // TODO: propagate errors
        // sensor reset not active
        // self.reset_pin.set_high().ok();

        // reset the spi bus on the sensor
        self.cs_pin.set_high();
        Timer::after_micros(50).await;
        self.cs_pin.set_low();
        Timer::after_micros(50).await;

        // Write to reset register
        self.write(reg::POWER_UP_RESET, 0x5A).await?;
        // 100 ms delay
        Timer::after_micros(100).await;

        // read registers 0x02 to 0x06 (and discard the data)
        self.read(reg::MOTION).await?;
        self.read(reg::DELTA_X_L).await?;
        self.read(reg::DELTA_X_H).await?;
        self.read(reg::DELTA_Y_L).await?;
        self.read(reg::DELTA_Y_H).await?;

        // upload the firmware
        self.upload_fw().await?;

        let is_valid_signature = self.check_signature().await.unwrap_or(false);

        // Write 0x00 (rest disable) to Config2 register for wired mouse or 0x20 for
        // wireless mouse design.
        self.write(reg::CONFIG_2, 0x00).await?;

        Timer::after_micros(100).await;

        Ok(is_valid_signature)
    }

    async fn power_up(&mut self) -> Result<(), Pmw3360Error<S>> {
        let is_valid_signature = self.power_up_inner().await.map_err(Pmw3360Error::Spi)?;
        if is_valid_signature {
            Ok(())
        } else {
            Err(Pmw3360Error::InvalidSignature)
        }
    }

    async fn upload_fw(&mut self) -> Result<(), S::Error> {
        // TODO: propagate errors
        // Write 0 to Rest_En bit of Config2 register to disable Rest mode.
        self.write(reg::CONFIG_2, 0x00).await?;

        // write 0x1d in SROM_enable reg for initializing
        self.write(reg::SROM_ENABLE, 0x1d).await?;

        // wait for 10 ms
        Timer::after_micros(10000).await;

        // write 0x18 to SROM_enable to start SROM download
        self.write(reg::SROM_ENABLE, 0x18).await?;

        // lower NCS
        self.cs_pin.set_low();

        // first byte is address
        self.spi
            .transfer_in_place(&mut [reg::SROM_LOAD_BURST | 0x80])
            .await?;
        Timer::after_micros(15).await;

        // send the rest of the firmware
        for element in srom_tracking::FW.iter() {
            self.spi.transfer_in_place(&mut [*element]).await?;
            Timer::after_micros(15).await;
        }

        Timer::after_micros(2).await;
        self.cs_pin.set_high();
        Timer::after_micros(200).await;
        Ok(())
    }
}

impl<'d, S: SpiBus + 'd, OP: OutputPin + 'd> Mouse for Pmw3360<'d, S, OP> {
    async fn init(&mut self) -> Result<(), rktk::interface::error::RktkError> {
        self.power_up().await.map_err(|_| {
            rktk::interface::error::RktkError::GeneralError("Failed to power up PMW3360")
        })?;

        Ok(())
    }

    async fn read(&mut self) -> Result<(i8, i8), rktk::interface::error::RktkError> {
        self.burst_read()
            .await
            .map(|data| (data.dx as i8, data.dy as i8))
            .map_err(|_| rktk::interface::error::RktkError::GeneralError("Failed to read PMW3360"))
    }

    async fn set_cpi(&mut self, cpi: u16) -> Result<(), rktk::interface::error::RktkError> {
        self.set_cpi(cpi).await;
        Ok(())
    }

    async fn get_cpi(&mut self) -> Result<u16, rktk::interface::error::RktkError> {
        Err(rktk::interface::error::RktkError::NotSupported)
    }
}
