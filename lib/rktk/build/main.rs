use std::{env, fs, path::Path};

use const_gen::*;
use schemars::schema_for;

mod rktk_json_docsrs;
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
                  "$schema": "../lib/rktk/schema.json",
                  "keyboard": {
                    "cols": 14,
                    "rows": 5,
                    "name": "test"
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
        println!("cargo:rerun-if-changed={}", config_path);
        fs::read_to_string(config_path).expect("Failed to read config file")
    };

    let out = generate(&config).unwrap();
    let gen_path = Path::new(env::var("OUT_DIR").unwrap().as_str()).join("config.rs");
    fs::write(&gen_path, out).expect("Failed to write generated code");
    std::process::Command::new("rustfmt")
        .arg(&gen_path)
        .output()
        .expect("Failed to run rustfmt");

    // println!("cargo:warning=Wrote generated code to {:?}", gen_path);
}

macro_rules! definitions {
    ($vec:tt, $($path:path),* ) => {
        $(
            $vec.push(const_definition!(#[derive(Debug, serde::Serialize)] pub $path));
        )*
    };
}

pub fn generate(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config: schema::Config = serde_json::from_str(value)?;

    let mut text = vec![
        "use rktk_keymanager::interface::state::config::*;".to_string(),
        "pub mod schema {".to_string(),
        "use rktk_keymanager::interface::state::config::*;".to_string(),
    ];
    definitions!(
        text,
        schema::Config,
        schema::keyboard::Keyboard,
        schema::rktk::RktkConfig,
        schema::key_manager::KeyManagerConfig,
        schema::key_manager::KeymanagerConstantConfig
    );
    text.push("}".to_string());
    text.push("use schema::*;".to_string());

    text.push(const_declaration!(
        #[doc = "Config generated from json"]
        pub CONFIG = config
    ));

    Ok(text.join("\n"))
}
