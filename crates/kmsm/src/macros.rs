use macro_rules_attribute::attribute_alias;

macro_rules! normal {
    ($name:ident, $type:ident, $variant:ident) => {
        pub const $name: crate::keycode::KeyAction =
            crate::keycode::KeyAction::Normal(crate::keycode::KeyCode::$type($type::$variant));
    };
}

macro_rules! with_consts {
    {
        $(#[$($attr:tt)*])*
        $vis:vis enum $name:ident {
            $(
                $(#[$($field_attr:tt)*])*
                $variant:ident $(= $val:literal)?,
            )*
        }
    } => {
        $(#[$($attr)*])*
        $vis enum $name {
            $(
                $(#[$($field_attr)*])*
                $variant $(= $val)?,
            )*
        }

        paste::paste! {
            $(
                pub const [<$variant:snake:upper>]: crate::keycode::KeyAction =
                    crate::keycode::KeyAction::Normal(
                        crate::keycode::KeyCode::$name($name::$variant)
                    );
            )*
        }
    };
}

macro_rules! impl_display {
    ($type:ty) => {
        use core::fmt::{self, Display, Formatter};
        impl Display for $type {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let s: &'static str = self.into();
                write!(f, "{s}")
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
        #[derive(PartialEq, Eq, Clone, Debug)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    ;
}

pub(super) use impl_display;
pub(super) use normal;
pub(super) use with_consts;
