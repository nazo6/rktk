use clap::ValueEnum;
use serde::Deserialize;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum BuildProfile {
    MinSize,
    MaxPerf,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum BuildMcu {
    Rp2040,
    Nrf52840,
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    pub profile: Option<BuildProfile>,
    pub mcu: Option<BuildMcu>,
}
