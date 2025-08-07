// HACK: Nested macro_rules
// https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
#[rustfmt::skip]
macro_rules! defmt_or_core {
    ($d:tt $name:ident) => {
        #[macro_export]
        macro_rules! $name {
            ($d($d x:tt)*) => {
                #[cfg(not(feature = "defmt"))]
                ::core::$name!($d($d x)*);

                #[cfg(feature = "defmt")]
                ::defmt::$name!($d($d x)*);
            }
        }
    }
}

#[rustfmt::skip]
macro_rules! defmt_or_log_or_noop {
    ($d:tt $name:ident) => {
        #[macro_export]
        macro_rules! $name {
            ($s:literal $d(, $d x:expr)* $d(,)?) => {
                #[cfg(feature = "defmt")]
                ::defmt::$name!($s $d(, $d x)*);

                #[cfg(feature = "log")]
                ::log::$name!($s $d(, $d x)*);

                #[cfg(all(not(feature = "defmt"), not(feature = "log")))]
                let _ = ($d( & $d x ),*);
            }
        }
    }
}

defmt_or_core!($ assert);
defmt_or_core!($ assert_eq);
defmt_or_core!($ assert_ne);
defmt_or_core!($ debug_assert);
defmt_or_core!($ debug_assert_eq);
defmt_or_core!($ debug_assert_ne);
defmt_or_core!($ todo);
defmt_or_core!($ unreachable);
defmt_or_core!($ panic);

defmt_or_log_or_noop!($ trace);
defmt_or_log_or_noop!($ debug);
defmt_or_log_or_noop!($ info);
defmt_or_log_or_noop!($ warn);
defmt_or_log_or_noop!($ error);

#[macro_export]
macro_rules! intern {
    ($s:literal) => {
        #[cfg(not(feature = "defmt"))]
        $s
        #[cfg(feature = "defmt")]
        ::defmt::intern!($s)
    };
}

#[macro_export]
macro_rules! unwrap {
    ($arg:expr) => {
        #[cfg(feature = "defmt")]
        ::defmt::unwrap!($arg)

        #[cfg(not(feature = "defmt"))]
        match $crate::macros::unwrap_helper::Try::into_result($arg) {
            ::core::result::Result::Ok(t) => t,
            ::core::result::Result::Err(e) => {
                ::core::panic!("unwrap of `{}` failed: {:?}", ::core::stringify!($arg), e);
            }
        }
    };
    ($arg:expr, $($msg:expr),+ $(,)? ) => {
        #[cfg(feature = "defmt")]
        ::defmt::unwrap!($arg:expr, $($msg:expr),+)

        #[cfg(not(feature = "defmt"))]
        match $crate::macros::unwrap_helper::Try::into_result($arg) {
            ::core::result::Result::Ok(t) => t,
            ::core::result::Result::Err(e) => {
                ::core::panic!("unwrap of `{}` failed: {}: {:?}", ::core::stringify!($arg), ::core::format_args!($($msg,)*), e);
            }
        }
    }
}

pub mod unwrap_helper {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct NoneError;

    pub trait Try {
        type Ok;
        type Error;
        fn into_result(self) -> Result<Self::Ok, Self::Error>;
    }

    impl<T> Try for Option<T> {
        type Ok = T;
        type Error = NoneError;

        #[inline]
        fn into_result(self) -> Result<T, NoneError> {
            self.ok_or(NoneError)
        }
    }

    impl<T, E> Try for Result<T, E> {
        type Ok = T;
        type Error = E;

        #[inline]
        fn into_result(self) -> Self {
            self
        }
    }
}
