#![doc = include_str!("../README.md")]
//!
//! This crate consists of the following modules:
//! - [`keycode`]: Keycode definitions
//! - [`keymap`]: Keymap definition
//! - [`state`]: State management
//!
//! To know how to define keymap, see `keycode` and `keymap` modules.

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![allow(non_snake_case)]

pub mod interface;
pub mod keycode;
pub mod keymap;
mod macros;
mod time;

#[cfg(any(test, feature = "state"))]
pub mod state;

#[cfg(feature = "schemars")]
mod generic_array_schema {
    use std::borrow::Cow;

    use schemars::{
        gen::SchemaGenerator,
        schema::{ArrayValidation, InstanceType, Schema, SchemaObject},
        JsonSchema,
    };

    pub struct GenericArray<T, const N: usize>([T; N]);

    impl<T: JsonSchema, const N: usize> JsonSchema for GenericArray<T, N> {
        fn schema_name() -> String {
            format!("Generic_Array_size_{}_of_{}", N, T::schema_name())
        }

        fn schema_id() -> Cow<'static, str> {
            format!("G[{}; {}]", N, T::schema_id()).into()
        }

        fn json_schema(gen: &mut SchemaGenerator) -> Schema {
            SchemaObject {
                instance_type: Some(InstanceType::Array.into()),
                array: Some(Box::new(ArrayValidation {
                    items: Some(gen.subschema_for::<T>().into()),
                    max_items: Some(N as u32),
                    min_items: Some(N as u32),
                    ..Default::default()
                })),
                ..Default::default()
            }
            .into()
        }
    }
}
