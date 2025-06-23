use std::{env, fs, path::Path};

use const_gen::*;
use schemars::schema_for;

mod schema;

fn main() {
    println!("cargo:rerun-if-env-changed=RKTK_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=DOCS_RS");

    let schema = schema_for!(schema::Config);

    let config = if std::env::var("DOCS_RS").is_ok() {
        // In docs.rs rktk.json cannot be loaded. So, loads pre-defined value.
        // Also, writing file is not allowed, so schema.json generation is also skipped.
        println!("cargo:warning=Using demo json for docs.rs");
        r#####"
            {
              "$schema": "./lib/rktk/schema.json",
              "constant": {
                "keyboard": {
                  "cols": 14,
                  "rows": 5
                }
              },
              "dynamic": {
                "keyboard": {
                  "name": "test-keyboard"
                }
              }
            }
        "#####
            .to_string()
    } else {
        fs::write(
            Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join("schema.json"),
            serde_json::to_string_pretty(&schema).expect("Failed to serialize schema"),
        )
        .expect("Failed to write schema.json");

        let config_path = env::var("RKTK_CONFIG_PATH").expect("RKTK_CONFIG_PATH is not set");
        println!("cargo:rerun-if-changed={config_path}");
        fs::read_to_string(config_path).expect("Failed to read config file")
    };

    let out = generate(&config).unwrap();
    let gen_path = Path::new(env::var("OUT_DIR").unwrap().as_str()).join("config.rs");
    fs::write(&gen_path, out).expect("Failed to write generated code");
    std::process::Command::new("rustfmt")
        .arg(&gen_path)
        .output()
        .expect("Failed to run rustfmt");
}

macro_rules! definitions {
    ($name:ident, $($path:path),* ) => {
        let $name = vec![$(
            const_definition!(#[derive(Debug, serde::Serialize)] pub $path),
        )*];
    };
}

pub fn generate(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config: schema::Config = serde_json::from_str(value)?;

    definitions!(
        code_schemas,
        schema::constant::ConstantConfig,
        schema::constant::BufferSizeConfig,
        schema::constant::KeyboardConstantConfig,
        schema::constant::KeymanagerConstantConfig,
        schema::dynamic::DynamicConfig,
        schema::dynamic::rktk::RktkConfig,
        schema::dynamic::rktk::RktkRgbConfig,
        schema::dynamic::keyboard::KeyboardConfig,
        schema::dynamic::key_manager::KeyManagerConfig
    );
    let code_schemas = code_schemas.join("\n");

    let code_const_config = const_declaration!(
        #[doc = "Config generated from `constant` key of json"]
        pub CONST_CONFIG = config.constant
    );

    let code_dynamic_config = const_declaration!(
        #[doc = "Config generated from `dynamic` key of json"]
        pub DYNAMIC_CONFIG_FROM_FILE = config.dynamic
    );

    let code = format!(
        r#"
        /// Configuration schema
        pub mod schema {{
            use rktk_keymanager::interface::state::config::*;

            {code_schemas}
        }}
        mod generate_config {{
            use super::schema::*;
            use rktk_keymanager::interface::state::config::*;

            {code_const_config}

            {code_dynamic_config}
        }}

        pub use generate_config::{{CONST_CONFIG, DYNAMIC_CONFIG_FROM_FILE}};
    "#
    );

    Ok(code)
}
