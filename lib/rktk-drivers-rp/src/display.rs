pub mod ssd1306 {
    use embassy_rp::i2c::Config;

    pub fn recommended_i2c_config() -> Config {
        let mut i2c_config = Config::default();
        i2c_config.frequency = 400_000;
        i2c_config
    }
}
