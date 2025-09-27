use std::{collections::HashMap, sync::LazyLock};

#[derive(serde::Deserialize)]
pub struct InternalCmdConfig {
    pub crates: HashMap<String, CrateConfig>,
    pub check_skip_global: Option<Vec<String>>,
    pub check_env: HashMap<String, String>,

    pub test_features_global: Option<Vec<String>>,

    pub doc_features_global: Option<Vec<String>>,

    pub publish_order: Vec<String>,
}

#[derive(Default, serde::Deserialize, Clone)]
#[serde(default)]
pub struct CrateConfig {
    // Check configuration
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

    // Test configuration
    pub test_enabled: bool,
    pub test_features: Option<Vec<String>>,

    // Doc configuration
    pub doc_enabled: bool,

    // Publish configuration
    /// If false, `_check` feature is used when publishing crate.
    /// If true, `_release` feature is used instead.
    pub use_release_feature: bool,
}

pub static CRATES_CONFIG: LazyLock<InternalCmdConfig> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../../crates_config.json"))
        .expect("Failed to parse crate_config.json")
});
