use macro_rules_attribute::attribute_alias;

pub mod key_manager;
pub mod keyboard;
pub mod rktk;

attribute_alias! {
    #[apply(common_derive)] =
        #[derive(serde::Deserialize, schemars::JsonSchema, const_gen::CompileConst)]
        #[serde(deny_unknown_fields)]
        #[inherit_doc]
    ;
}

/// Root configuration struct
#[derive(serde::Deserialize, schemars::JsonSchema, const_gen::CompileConst)]
#[inherit_doc]
pub struct Config {
    pub keyboard: keyboard::Keyboard,
    #[serde(default)]
    pub rktk: rktk::RktkConfig,
    #[serde(default)]
    pub key_manager: key_manager::KeyManagerConfig,
}
