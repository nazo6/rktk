#![allow(unused_mut)]
#![allow(unused)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Ident, parse_macro_input};

// private macros

#[proc_macro]
pub fn defmt_or_core(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let expanded = quote! {
        #[macro_export]
        macro_rules! #name {
            ($($x:tt)*) => {
                #[cfg(not(feature = "defmt"))]
                ::core::#name!($($x)*);

                #[cfg(feature = "defmt")]
                ::core::#name!($($x)*);
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn defmt_or_log_or_noop(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let expanded = quote! {
        #[macro_export]
        macro_rules! #name {
            ($s:literal $(, $x:expr)* $(,)?) => {{
                #[cfg(feature = "defmt")]
                {
                    ::defmt::#name!($s $(, $x)*);
                }
                #[cfg(feature = "log")]
                {
                    ::log::#name!($s $(, $x)*);
                }
                #[cfg(all(not(feature = "defmt"), not(feature = "log")))]
                {
                    let _ = ($( & $x ),*);
                }
            }};
        }
    };

    TokenStream::from(expanded)
}
