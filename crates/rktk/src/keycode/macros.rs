macro_rules! with_consts {
    {
        $key_type:ident,
        $(#[$($attr:tt)*])*
        $vis:vis enum $name:ident {
            $($variant:ident = $val:literal,)*
        }
    } => {
        $(#[$($attr)*])*
        $vis enum $name { $($variant = $val,)* }

        use super::{KeyAction, KeyDef, KeyCode};

        paste::paste!{
            $(pub const [<$variant:snake:upper>] :KeyDef = KeyDef::Key(KeyAction::Tap(KeyCode::$key_type($name::$variant)));)*
        }
    }
}

macro_rules! with_consts_no_val {
    {
        $key_type:ident,
        $(#[$($attr:tt)*])*
        $vis:vis enum $name:ident {
            $($variant:ident,)*
        }
    } => {
        $(#[$($attr)*])*
        $vis enum $name { $($variant,)* }

        use super::{KeyAction, KeyDef, KeyCode};

        paste::paste!{
            $(pub const [<$variant:snake:upper>] :KeyDef = KeyDef::Key(KeyAction::Tap(KeyCode::$key_type($name::$variant)));)*
        }
    }
}

macro_rules! normal {
    ($name:ident, $type:ident, $variant:ident) => {
        pub const $name: KeyDef = KeyDef::Key(KeyAction::Tap(KeyCode::$type($type::$variant)));
    };
}

macro_rules! bit_normal {
    ($name:ident, $type:ident, $variant:ident) => {
        pub const $name: KeyDef = KeyDef::Key(KeyAction::Tap(KeyCode::$type($type::$variant)));
    };
}

pub(super) use bit_normal;
pub(super) use normal;
pub(super) use with_consts;
pub(super) use with_consts_no_val;
