#![allow(unused_mut)]
#![allow(unused)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_attribute]
pub fn maybe_derive_format(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    #[cfg(feature = "_defmt")]
    let expanded = quote! {
        #[cfg_attr(feature="defmt", derive(defmt::Format))]
        #input
    };

    #[cfg(not(feature = "_defmt"))]
    let expanded = quote! {};

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn derive_format_and_debug(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut stmt = quote! {#[derive(core::fmt::Debug)]};
    let mut defmt_import = quote! {};

    #[cfg(feature = "_defmt")]
    {
        stmt = quote! {
            #stmt
            #[derive(rktk_log::__reexports::defmt::Format)]
        };
        defmt_import = quote! {use rktk_log::__reexports::defmt;};
    }

    let mod_name = format_ident!("dfag_hygine_{}", input.ident);
    let vis = input.vis.clone();

    TokenStream::from(quote! {
        mod #mod_name {
            #defmt_import
            use super::*;

            #stmt
            #input
        }

        #vis use #mod_name::*;
    })
}

// private macros

#[proc_macro]
pub fn defmt_or_core(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    #[cfg(feature = "_defmt")]
    let stmt = quote! {{
        pub use $crate::__reexports::defmt as defmt;
        $crate::__reexports::defmt::#name!($($x)*);
    }};

    #[cfg(not(feature = "_defmt"))]
    let stmt = quote! {::core::#name!($($x)*);};

    let expanded = quote! {
        #[macro_export]
        macro_rules! #name {
            ($($x:tt)*) => {
                #stmt
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn defmt_or_log_or_noop(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let mut log_count = 0;
    let mut stmt = quote! {};

    #[cfg(feature = "_defmt")]
    {
        log_count += 1;
        stmt = quote! {
            #stmt
            {
                pub use $crate::__reexports::defmt as defmt;
                $crate::__reexports::defmt::#name!($s $(, $x)*);
            };
        };
    }

    #[cfg(feature = "_log")]
    {
        log_count += 1;
        stmt = quote! {
            #stmt
            $crate::__reexports::log::#name!($s $(, $x)*);
        };
    }

    if log_count == 0 {
        stmt = quote! {
            let _ = ($( & $x ),*);
        };
    }
    let expanded = quote! {
        #[macro_export]
        macro_rules! #name {
            ($s:literal $(, $x:expr)* $(,)?) => {
                #stmt
            };
        }
    };

    TokenStream::from(expanded)
}
