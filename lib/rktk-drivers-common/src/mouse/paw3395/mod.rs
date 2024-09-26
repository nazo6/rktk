#![allow(dead_code)]

pub mod config;
mod error;
mod power_up;
mod registers;

use embassy_time::Timer;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;
use error::Paw3395Error;
use registers as reg;
use rktk::interface::{mouse::MouseDriver, DriverBuilder};

#[derive(Default)]
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

pub struct Paw3395Builder<'d, S: SpiBus + 'd, OP: OutputPin + 'd> {
    spi: S,
    cs_pin: OP,
    config: config::Config,
    _marker: core::marker::PhantomData<&'d ()>,
}

impl<'d, S: SpiBus + 'd, OP: OutputPin + 'd> Paw3395Builder<'d, S, OP> {
    pub fn new(spi: S, cs_pin: OP, config: config::Config) -> Self {
        Self {
            spi,
            cs_pin,
            config,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<'d, S: SpiBus + 'd, OP: OutputPin + 'd> DriverBuilder for Paw3395Builder<'d, S, OP> {
    type Output = Paw3395<'d, S, OP>;

    type Error = Paw3395Error<S::Error, OP::Error>;

    async fn build(self) -> Result<Self::Output, Self::Error> {
        let mut driver = Paw3395 {
            _marker: core::marker::PhantomData,
            spi: self.spi,
            cs_pin: self.cs_pin,
        };

        driver.power_up(self.config).await?;

        Ok(driver)
    }
}

pub struct Paw3395<'d, S: SpiBus + 'd, OP: OutputPin + 'd> {
    _marker: core::marker::PhantomData<&'d ()>,
    spi: S,
    cs_pin: OP,
}

impl<'d, S: SpiBus + 'd, OP: OutputPin + 'd> MouseDriver for Paw3395<'d, S, OP> {
    async fn read(&mut self) -> Result<(i8, i8), rktk::interface::error::RktkError> {
        self.burst_read()
            .await
            .map(|data| (data.dx as i8, data.dy as i8))
            .map_err(|_| rktk::interface::error::RktkError::GeneralError("Failed to read PAW3395"))
    }

    async fn set_cpi(&mut self, cpi: u16) -> Result<(), rktk::interface::error::RktkError> {
        self.set_cpi(cpi).await.map_err(|_| {
            rktk::interface::error::RktkError::GeneralError("Failed to set cpi to PAW3395")
        })?;
        Ok(())
    }

    async fn get_cpi(&mut self) -> Result<u16, rktk::interface::error::RktkError> {
        Err(rktk::interface::error::RktkError::NotSupported)
    }
}

impl<'d, S: SpiBus + 'd, OP: OutputPin + 'd> Paw3395<'d, S, OP> {
    pub async fn burst_read(&mut self) -> Result<BurstData, Paw3395Error<S::Error, OP::Error>> {
        self.cs_pin.set_low().map_err(Paw3395Error::Gpio)?;

        // tNCS-SCLK
        Timer::after_micros(2).await;

        self.spi
            .transfer_in_place(&mut [reg::MOTION_BURST])
            .await
            .map_err(Paw3395Error::Spi)?;

        // tSRAD
        Timer::after_micros(2).await;

        // Read the 12 bytes of burst data
        let mut buf = [0u8; 12];
        for b in buf.iter_mut() {
            let t_buf = &mut [0x00];
            match self.spi.transfer_in_place(t_buf).await {
                Ok(()) => *b = *t_buf.first().unwrap(),
                Err(_) => return Err(Paw3395Error::General("Failed to read burst data")),
            }
        }

        // Raise NCS
        self.cs_pin.set_high().map_err(Paw3395Error::Gpio)?;

        // NOTE: Same as tSRAD_MOTBR. temporary disabled.
        //
        // tBEXIT
        Timer::after_micros(1).await;

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

    pub async fn set_cpi(&mut self, cpi: u16) -> Result<(), Paw3395Error<S::Error, OP::Error>> {
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

    pub async fn check_signature(&mut self) -> Result<bool, Paw3395Error<S::Error, OP::Error>> {
        let pid = self.read(reg::PRODUCT_ID).await.unwrap_or(0);
        let ipid = self.read(reg::INV_PRODUCT_ID).await.unwrap_or(0);

        Ok(pid == 0x51 && ipid == 0xAE)
    }

    async fn write(
        &mut self,
        address: u8,
        data: u8,
    ) -> Result<(), Paw3395Error<S::Error, OP::Error>> {
        self.cs_pin.set_low().map_err(Paw3395Error::Gpio)?;
        // tNCS-SCLK
        Timer::after_micros(1).await;

        self.spi
            .transfer_in_place(&mut [address | 0x80])
            .await
            .map_err(Paw3395Error::Spi)?;
        self.spi
            .transfer_in_place(&mut [data])
            .await
            .map_err(Paw3395Error::Spi)?;

        // tSCLK-NCS (write)
        Timer::after_micros(35).await;
        self.cs_pin.set_high().map_err(Paw3395Error::Gpio)?;

        // tSWW/tSWR minus tSCLK-NCS (write)
        Timer::after_micros(145).await;

        Ok(())
    }

    async fn read(&mut self, address: u8) -> Result<u8, Paw3395Error<S::Error, OP::Error>> {
        self.cs_pin.set_low().map_err(Paw3395Error::Gpio)?;
        // tNCS-SCLK
        Timer::after_micros(1).await;

        // send adress of the register, with MSBit = 0 to indicate it's a read
        self.spi
            .transfer_in_place(&mut [address & 0x7f])
            .await
            .map_err(Paw3395Error::Spi)?;

        // tSRAD
        Timer::after_micros(160).await;

        let mut ret = 0;
        let mut buf = [0x00];
        if (self.spi.transfer_in_place(&mut buf).await).is_ok() {
            ret = *buf.first().unwrap();
        }

        // tSCLK-NCS (read)
        Timer::after_micros(1).await;
        self.cs_pin.set_high().map_err(Paw3395Error::Gpio)?;

        //  tSRW/tSRR minus tSCLK-NCS
        Timer::after_micros(20).await;

        Ok(ret)
    }

    async fn power_up(
        &mut self,
        config: config::Config,
    ) -> Result<(), Paw3395Error<S::Error, OP::Error>> {
        self.cs_pin.set_high().map_err(Paw3395Error::Gpio)?;
        Timer::after_micros(50).await;
        self.cs_pin.set_low().map_err(Paw3395Error::Gpio)?;
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
        for (addr, data) in config.mode.commands.iter() {
            self.write(*addr, *data).await?;
        }
        if let Some(c) = config.mode._0x40 {
            let mut _0x40 = self.read(0x40).await?;
            _0x40 |= c;
            self.write(0x40, _0x40).await?;
        }

        // set lift cutoff config
        self.write(0x7F, 0x0C).await?;
        let lift_config = self.read(0x4E).await?;
        self.write(0x7F, 0x00).await?;
        let lift_config = lift_config | config.lift_cutoff as u8;
        self.write(0x7F, 0x0C).await?;
        self.write(0x4E, lift_config).await?;
        self.write(0x7F, 0x00).await?;

        Ok(())
    }
}
