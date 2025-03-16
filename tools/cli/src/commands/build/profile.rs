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
    #[serde(rename = "opt-level", serialize_with = "serialize_opt_level")]
    pub opt_level: Option<String>,
    pub lto: Option<String>,
    pub panic: Option<String>,
    #[serde(rename = "codegen-units")]
    pub codegen_units: Option<u32>,
    pub strip: Option<bool>,
    pub rustflags: Option<Vec<String>>,
    pub debug: Option<bool>,
}

fn serialize_opt_level<S: serde::Serializer>(
    val: &Option<String>,
    s: S,
) -> Result<S::Ok, S::Error> {
    if let Some(val) = val {
        if let Ok(level_num) = val.parse::<u32>() {
            s.serialize_u32(level_num)
        } else {
            s.serialize_str(val)
        }
    } else {
        s.serialize_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoCmd {
    pub build_std: Option<String>,
    pub build_std_features: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfigToml {
    profile: HashMap<String, CargoProfile>,
}

macro_rules! gen_profile {
    ($($name:ident: $val:tt,)*) => {
        paste::paste! {
            #[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
            pub enum BuildProfileList {
                $([<$name>],)*
            }

            impl BuildProfileList {
                pub fn get_profile(&self) -> &'static Profile {
                    match self {
                        $(BuildProfileList::$name => &*[<PROFILE_ $name:snake:upper>],)*
                    }
                }
            }

            pub static PROFILE_CONFIG_TOML: LazyLock<ProfileConfigToml> = LazyLock::new(|| ProfileConfigToml {
                profile: {
                    let mut map = HashMap::new();
                    $(
                        map.insert(
                            stringify!([<$name:snake>]).to_string(),
                            [<PROFILE_ $name:snake:upper>].cargo_profile.clone(),
                        );
                    )*
                    map
                },
            });
        }

        $(
            gen_profile!(@profile, $name: $val,);
        )*
    };
    (@profile, $name:ident: $val:tt,) => {
        paste::paste! {
            pub static [<PROFILE_ $name:snake:upper>]: LazyLock<Profile> = LazyLock::new(|| Profile $val);
        }
    }
}

gen_profile!(
    MinSize: {
        name: "min_size".to_string(),
        cargo_profile: CargoProfile {
            inherits: "release".to_string(),
            opt_level: Some("z".to_string()),
            lto: Some("fat".to_string()),
            panic: Some("abort".to_string()),
            codegen_units: Some(1),
            strip: Some(false),
            rustflags: Some(vec!["-Zlocation-detail=none".to_string()]),
            debug: Some(true),
        },
        cargo_cmd: CargoCmd {
            build_std: Some("core,alloc,panic_abort".to_string()),
            build_std_features: Some("panic_immediate_abort,optimize_for_size".to_string()),
        },
    },
    MinSizePanicMessage: {
        name: "min_size_panic_message".to_string(),
        cargo_profile: CargoProfile {
            inherits: "release".to_string(),
            opt_level: Some("z".to_string()),
            lto: Some("fat".to_string()),
            panic: None,
            codegen_units: Some(1),
            strip: Some(false),
            rustflags: None,
            debug: Some(false),
        },
        cargo_cmd: CargoCmd {
            build_std: Some("core".to_string()),
            build_std_features: Some("optimize_for_size".to_string()),
        },
    },
    MaxPerf: {
        name: "max_perf".to_string(),
        cargo_profile: CargoProfile {
            inherits: "release".to_string(),
            opt_level: Some("3".to_string()),
            lto: Some("thin".to_string()),
            panic: None,
            codegen_units: Some(1),
            strip: Some(false),
            rustflags: None,
            debug: Some(true),
        },
        cargo_cmd: CargoCmd {
            build_std: Some("core,alloc".to_string()),
            build_std_features: None,
        },
    },
);
