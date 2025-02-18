use rktk_log_macros::{defmt_or_core, defmt_or_log_or_noop};

defmt_or_core!(assert);
defmt_or_core!(assert_eq);
defmt_or_core!(assert_ne);
defmt_or_core!(debug_assert);
defmt_or_core!(debug_assert_eq);
defmt_or_core!(debug_assert_ne);
defmt_or_core!(todo);
defmt_or_core!(unreachable);
defmt_or_core!(panic);

defmt_or_log_or_noop!(trace);
defmt_or_log_or_noop!(debug);
defmt_or_log_or_noop!(info);
defmt_or_log_or_noop!(warn);
defmt_or_log_or_noop!(error);

#[macro_export]
macro_rules! intern {
    ($s:literal) => {
        #[cfg(not(feature = "defmt"))]
        $s
        #[cfg(feature = "defmt")]
        {
            pub use $crate::__reexports::defmt as defmt;
            $crate::__reexports::defmt::intern!($s)
        }
    };
}

#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! unwrap {
    ($($x:tt)*) => {{
        pub use $crate::__reexports::defmt as defmt;
        $crate::__reexports::defmt::unwrap!($($x)*)
    }};
}

#[cfg(not(feature = "defmt"))]
#[macro_export]
macro_rules! unwrap {
    ($arg:expr) => {
        match $crate::macros::unwrap_helper::Try::into_result($arg) {
            ::core::result::Result::Ok(t) => t,
            ::core::result::Result::Err(e) => {
                ::core::panic!("unwrap of `{}` failed: {:?}", ::core::stringify!($arg), e);
            }
        }
    };
    ($arg:expr, $($msg:expr),+ $(,)? ) => {
        match $crate::macros::unwrap_helper::Try::into_result($arg) {
            ::core::result::Result::Ok(t) => t,
            ::core::result::Result::Err(e) => {
                ::core::panic!("unwrap of `{}` failed: {}: {:?}", ::core::stringify!($arg), ::core::format_args!($($msg,)*), e);
            }
        }
    }
}

#[cfg(not(feature = "defmt"))]
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
