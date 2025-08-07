pub mod ssd1306 {
    use embassy_nrf::twim::{Config, Frequency};

    /// Returns a recommended TWIM configuration for SSD1306.
    pub fn recommended_i2c_config() -> Config {
        let mut i2c_config = Config::default();
        i2c_config.frequency = Frequency::K400;
        i2c_config
    }
}
