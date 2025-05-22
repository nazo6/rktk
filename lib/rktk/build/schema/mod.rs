#![allow(private_interfaces)]

use macro_rules_attribute::attribute_alias;

pub mod constant;
pub mod dynamic;

attribute_alias! {
    #[apply(common_derive)] =
        #[derive(serde::Deserialize, schemars::JsonSchema, const_gen::CompileConst)]
        #[serde(deny_unknown_fields)]
        #[inherit_docs]
    ;
}

#[derive(serde::Deserialize, schemars::JsonSchema, const_gen::CompileConst)]
#[inherit_docs]
pub struct Config {
    pub constant: constant::ConstantConfig,
    pub dynamic: dynamic::DynamicConfig,
}
