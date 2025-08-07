#![no_std]

pub mod helper;
#[doc(hidden)]
pub mod macros;

#[cfg(not(feature = "defmt"))]
pub trait MaybeFormat: core::fmt::Debug {}
#[cfg(not(feature = "defmt"))]
impl<T> MaybeFormat for T where T: core::fmt::Debug {}

#[cfg(feature = "defmt")]
pub trait MaybeFormat: core::fmt::Debug + defmt::Format {}
#[cfg(feature = "defmt")]
impl<T> MaybeFormat for T where T: core::fmt::Debug + defmt::Format {}
