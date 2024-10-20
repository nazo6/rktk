use std::{collections::HashMap, sync::LazyLock};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub cargo_profile: CargoProfile,
    pub cargo_cmd: CargoCmd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoProfile {
    pub inherits: String,
    #[serde(rename = "opt-level")]
    pub opt_level: Option<String>,
    pub lto: Option<String>,
    pub panic: Option<String>,
    #[serde(rename = "codegen-units")]
    pub codegen_units: Option<u32>,
    pub strip: Option<bool>,
    pub rustflags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoCmd {
    pub build_std: Option<String>,
    pub build_std_features: Option<String>,
}

pub static PROFILE_MIN_SIZE: LazyLock<Profile> = LazyLock::new(|| Profile {
    name: "min-size".to_string(),
    cargo_profile: CargoProfile {
        inherits: "release".to_string(),
        opt_level: Some("z".to_string()),
        lto: Some("fat".to_string()),
        panic: Some("abort".to_string()),
        codegen_units: Some(1),
        strip: Some(true),
        rustflags: Some(vec!["-Zlocation-detail=none".to_string()]),
    },
    cargo_cmd: CargoCmd {
        build_std: Some("core,panic_abort".to_string()),
        build_std_features: Some("panic_immediate_abort".to_string()),
    },
});

pub static PROFILE_MAX_PERF: LazyLock<Profile> = LazyLock::new(|| Profile {
    name: "max-perf".to_string(),
    cargo_profile: CargoProfile {
        inherits: "release".to_string(),
        opt_level: Some("3".to_string()),
        lto: Some("thin".to_string()),
        panic: None,
        codegen_units: Some(1),
        strip: Some(true),
        rustflags: None,
    },
    cargo_cmd: CargoCmd {
        build_std: Some("core".to_string()),
        build_std_features: None,
    },
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfigToml {
    profile: HashMap<String, CargoProfile>,
}

pub static PROFILE_CONFIG_TOML: LazyLock<ProfileConfigToml> = LazyLock::new(|| ProfileConfigToml {
    profile: {
        let mut map = HashMap::new();
        map.insert(
            "min-size".to_string(),
            PROFILE_MIN_SIZE.cargo_profile.clone(),
        );
        map.insert(
            "max-perf".to_string(),
            PROFILE_MAX_PERF.cargo_profile.clone(),
        );
        map
    },
});
