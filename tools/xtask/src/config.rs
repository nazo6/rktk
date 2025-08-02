use std::{collections::HashMap, sync::LazyLock};

#[derive(serde::Deserialize)]
pub struct InternalCmdConfig {
    pub crates: HashMap<String, CrateConfig>,
    pub check_skip_global: Option<Vec<String>>,
    pub check_env: HashMap<String, String>,

    pub test_features_global: Option<Vec<String>>,

    pub doc_features_global: Option<Vec<String>>,
}

#[derive(Default, serde::Deserialize, Clone)]
#[serde(default)]
pub struct CrateConfig {
    /// Disables feature powerset check
    pub check_no_powerset: bool,
    /// Features to check (these features will be always added).
    pub check_features: Option<Vec<String>>,
    pub check_at_least_one_of: Option<Vec<Vec<String>>>,
    pub check_group_features: Option<Vec<Vec<String>>>,
    pub check_mutually_exclusive_features: Option<Vec<Vec<String>>>,
    /// Features to skip check (these features will be never added.). This overrides global skip.
    pub check_skip: Option<Vec<String>>,
    /// If false, check will be performed through `cargo clippy`.
    /// If true, `cargo build` will be used instead.`
    pub check_build: bool,

    pub test_enabled: bool,
    pub test_features: Option<Vec<String>>,

    pub doc_disabled: bool,
}

pub static CRATES_CONFIG: LazyLock<InternalCmdConfig> = LazyLock::new(|| {
    toml::from_str(include_str!("../../../crates_config.toml"))
        .expect("Failed to parse CratesConfig.toml")
});
