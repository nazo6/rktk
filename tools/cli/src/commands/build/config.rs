use serde::Deserialize;

use super::mcu::BuildMcuList;
use super::profile::BuildProfileList;

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    pub profile: Option<BuildProfileList>,
    pub mcu: Option<BuildMcuList>,
}
