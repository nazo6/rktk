//! Hooks traits

#![allow(async_fn_in_trait)]

pub mod dongle;
pub mod master;
pub mod rgb;

pub use common::CommonHooks;
pub use master::MasterHooks;
pub use rgb::RgbHooks;
pub use slave::SlaveHooks;

mod common {
    use crate::{
        drivers::interface::{keyscan::KeyscanDriver, mouse::MouseDriver, storage::StorageDriver},
        config::Hand,
    };

    /// Hooks common for both master and slave side
    pub trait CommonHooks {
        async fn on_init(
            &mut self,
            _hand: Hand,
            _key_scanner: &mut impl KeyscanDriver,
            _mouse: Option<&mut impl MouseDriver>,
            _storage: Option<&mut impl StorageDriver>,
        ) {
        }
    }
}

mod slave {
    use crate::drivers::interface::{keyscan::KeyscanDriver, mouse::MouseDriver};

    pub trait SlaveHooks {
        /// Called after slave side initialization.
        async fn on_slave_init(
            &mut self,
            _key_scanner: &mut impl KeyscanDriver,
            _mouse: Option<&mut impl MouseDriver>,
        ) {
        }
    }
}
