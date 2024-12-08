use macro_rules_attribute::attribute_alias;

macro_rules! normal {
    ($name:ident, $type:ident, $variant:ident) => {
        pub const $name: crate::keycode::KeyAction =
            crate::keycode::KeyAction::Normal(crate::keycode::KeyCode::$type($type::$variant));
    };
}

macro_rules! with_consts {
    // for enum with value
    {
        $(#[$($attr:tt)*])*
        $vis:vis enum $name:ident {
            $($variant:ident = $val:literal,)*
        }
    } => {
        $(#[$($attr)*])*
        $vis enum $name { $($variant = $val,)* }

        paste::paste!{
            $(pub const [<$variant:snake:upper>] : crate::keycode::KeyAction = crate::keycode::KeyAction::Normal(crate::keycode::KeyCode::$name($name::$variant));)*
        }
    };
    // for enum without value
    {
        $(#[$($attr:tt)*])*
        $vis:vis enum $name:ident {
            $($variant:ident,)*
        }
    } => {
        $(#[$($attr)*])*
        $vis enum $name { $($variant,)* }

        paste::paste!{
            $(pub const [<$variant:snake:upper>] : crate::keycode::KeyAction = crate::keycode::KeyAction::Normal(crate::keycode::KeyCode::$name($name::$variant));)*
        }
    }
}

macro_rules! impl_display_bitflags {
    ($type:ty) => {
        use core::fmt::{self, Display, Formatter};
        impl Display for $type {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                bitflags::parser::to_writer(self, f)
            }
        }
    };
}

attribute_alias! {
    #[apply(common_derive)] =
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(
            feature = "postcard",
            derive(postcard::experimental::max_size::MaxSize)
        )]
        #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
        #[derive(PartialEq, Eq, Clone, Debug)]
    ;
}

pub(super) use impl_display_bitflags;
pub(super) use normal;
pub(super) use with_consts;
