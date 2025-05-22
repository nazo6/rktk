pub mod key_manager;
pub mod keyboard;
pub mod rktk;

/// Root configuration struct
///
/// JSON schema of config is available at `schema.json`.
#[derive(serde::Deserialize, schemars::JsonSchema, const_gen::CompileConst)]
#[inherit_docs]
pub struct DynamicConfig {
    pub keyboard: keyboard::KeyboardConfig,
    #[serde(default)]
    pub rktk: rktk::RktkConfig,
    #[serde(default)]
    pub key_manager: key_manager::KeyManagerConfig,
}
