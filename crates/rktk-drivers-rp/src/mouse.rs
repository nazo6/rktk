pub mod paw3395 {
    use embassy_rp::spi::{Config, Phase, Polarity};

    pub fn recommended_spi_config() -> Config {
        let mut spi_config = Config::default();
        spi_config.frequency = 7_000_000;
        spi_config.polarity = Polarity::IdleHigh;
        spi_config.phase = Phase::CaptureOnSecondTransition;

        spi_config
    }
}
pub mod pmw3360 {
    use embassy_rp::spi::{Config, Phase, Polarity};

    pub fn recommended_spi_config() -> Config {
        let mut spi_config = Config::default();
        spi_config.frequency = 7_000_000;
        spi_config.polarity = Polarity::IdleHigh;
        spi_config.phase = Phase::CaptureOnSecondTransition;

        spi_config
    }
}
