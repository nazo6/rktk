use std::{collections::HashMap, sync::LazyLock};

#[derive(serde::Deserialize)]
pub struct InternalCmdConfig {
    pub crates: HashMap<String, CrateConfig>,
    pub check_skip_global: Option<Vec<String>>,
    pub test_features_global: Option<Vec<String>>,
}

#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct CrateConfig {
    /// Disables feature powerset check
    pub check_no_powerset: bool,
    /// Features to check (these features will be always added).
    pub check_features: Option<Vec<String>>,
    pub check_at_least_one_of: Option<Vec<String>>,
    /// Features to skip check (these features will be never added.). This overrides global skip.
    ///
    pub check_skip: Option<Vec<String>>,
    pub test_enabled: bool,
}

pub const CRATES_CONFIG: LazyLock<InternalCmdConfig> = LazyLock::new(|| {
    toml::from_str(include_str!("./crates_config.toml")).expect("Failed to parse CratesConfig.toml")
});
