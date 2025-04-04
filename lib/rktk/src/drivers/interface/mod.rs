//! Driver interface types
#![allow(async_fn_in_trait)]

pub mod ble;
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

macro_rules! generate_builder {
    ($driver:ident) => {
        paste::paste! {
        pub trait [<$driver Builder>] {
            type Output: $driver;
            type Error: core::fmt::Debug + rktk_log::MaybeFormat;
            async fn build(self) -> Result<(Self::Output, impl Future<Output = ()> + 'static), Self::Error>;
        }
        }
    };
}
pub(crate) use generate_builder;
