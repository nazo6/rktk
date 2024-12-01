use schemars::schema_for;

mod rktk_json_docsrs;
mod ser_codegen;

include!("../src/config/static_config/schema.rs");

fn main() {
    let config_path = if std::env::var("DOCS_RS").is_ok() {
        // in docs.rs rktk.json cannot be loaded.
        String::new()
    } else {
        std::env::var("RKTK_CONFIG_PATH").expect("RKTK_CONFIG_PATH is not set")
    };

    // println!("cargo:warning=Using config: {}", config_path);
    // println!("cargo:warning=current_time:{:?}", std::time::Instant::now());

    println!("cargo:rerun-if-env-changed=RKTK_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-changed={}", config_path);
    println!("cargo:rerun-if-changed=src/config/static_config/schema.rs");

    println!("cargo:rustc-cfg=no_build");

    let config = if std::env::var("DOCS_RS").is_ok() {
        // in docs.rs use demo config insted.
        println!("cargo:warning=Using demo json for docs.rs");
        rktk_json_docsrs::RKTK_JSON_DOCSRS.to_string()
    } else {
        std::fs::read_to_string(config_path).expect("Failed to read config file")
    };

    let config: StaticConfig = serde_json::from_str(&config).expect("Failed to parse config file");

    let code = format!(
        "pub use schema::*;pub const CONFIG: StaticConfig = {};",
        ser_codegen::to_string(&config).unwrap()
    );

    let gen_path = std::path::Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join("gen.rs");
    std::fs::write(&gen_path, code).expect("Failed to write generated code");

    // println!("cargo:warning=Wrote generated code to {:?}", gen_path);

    if std::env::var("DOCS_RS").is_err() {
        let schema = schema_for!(StaticConfig);
        std::fs::write(
            std::path::Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
                .join("schema.json"),
            serde_json::to_string_pretty(&schema).expect("Failed to serialize schema"),
        )
        .expect("Failed to write schema.json");
    }
}
