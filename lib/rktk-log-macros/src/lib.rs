extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, visit_mut::VisitMut, DeriveInput, Expr, Ident, Item};

#[proc_macro_attribute]
pub fn maybe_derive_debug(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {
        #[cfg_attr(feature="log", derive(core::fmt::Debug))]
        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn maybe_derive_format(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {
        #[cfg_attr(feature="defmt", derive(defmt::Format))]
        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn derive_format_or_debug(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        #[cfg_attr(feature="defmt", derive(defmt::Format))]
        #[cfg_attr(feature="log", derive(core::fmt::Debug))]
        #input
    };

    TokenStream::from(expanded)
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

    #[allow(unused_mut)]
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

    println!("{}", expanded.to_string());

    TokenStream::from(expanded)
}
