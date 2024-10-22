use clap::ValueEnum;
use serde::Deserialize;

use super::profile::BuildProfileList;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum BuildMcu {
    Rp2040,
    Nrf52840,
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    pub profile: Option<BuildProfileList>,
    pub mcu: Option<BuildMcu>,
}
