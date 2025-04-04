//! Driver interface types
#![allow(async_fn_in_trait)]

pub mod debounce;
pub mod display;
pub mod dongle;
pub mod encoder;
pub mod keyscan;
pub mod mouse;
pub mod reporter;
pub mod rgb;
pub mod split;
pub mod storage;
pub mod system;
pub mod usb;
pub mod wireless;

pub trait Error: core::fmt::Debug + rktk_log::MaybeFormat {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

#[non_exhaustive]
pub enum ErrorKind {
    NotSupported,
    Other,
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        unreachable!()
    }
}

macro_rules! generate_builder {
    ($driver:ident) => {
        paste::paste! {
        pub trait [<$driver Builder>] {
            type Output: $driver;
            type Error: rktk_log::MaybeFormat;
            async fn build(self) -> Result<(Self::Output, impl Future<Output = ()> + 'static), Self::Error>;
        }
        }
    };
}
pub(crate) use generate_builder;
