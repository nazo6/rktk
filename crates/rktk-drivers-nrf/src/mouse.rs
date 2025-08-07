pub mod paw3395 {
    use embassy_nrf::spim;

    pub fn recommended_spi_config() -> spim::Config {
        let mut config = spim::Config::default();
        config.frequency = spim::Frequency::M8;
        config.mode.polarity = spim::Polarity::IdleHigh;
        config.mode.phase = spim::Phase::CaptureOnSecondTransition;
        config
    }
}
pub mod pmw3360 {
    use embassy_nrf::spim;

    pub fn recommended_spi_config() -> spim::Config {
        let mut config = spim::Config::default();
        config.frequency = spim::Frequency::M8;
        config.mode.polarity = spim::Polarity::IdleHigh;
        config.mode.phase = spim::Phase::CaptureOnSecondTransition;
        config
    }
}
