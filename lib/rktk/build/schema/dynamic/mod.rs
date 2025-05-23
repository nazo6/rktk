pub mod key_manager;
pub mod keyboard;
pub mod rktk;

/// Root struct of the "dynamic" config
#[derive(serde::Deserialize, schemars::JsonSchema, const_gen::CompileConst)]
#[inherit_docs]
pub struct DynamicConfig {
    pub keyboard: keyboard::KeyboardConfig,
    #[serde(default)]
    pub rktk: rktk::RktkConfig,
    #[serde(default)]
    pub key_manager: key_manager::KeyManagerConfig,
}
