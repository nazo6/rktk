//! Hooks are used to customize the behavior of the application.

use interface::*;

pub mod interface;
pub use empty_hooks::create_empty_hooks;

/// Hooks that can be passed to [`crate::task::start`] function.
/// See earch trait's documentation for more information.
pub struct Hooks<CH: CommonHooks, MH: MasterHooks, SH: SlaveHooks, RH: RgbHooks> {
    pub common: CH,
    pub master: MH,
    pub slave: SH,
    pub rgb: RH,
}

/// Makes it easier to pass around all hooks as a single type.
///
/// Without this trait, users would need to write out the full type of hooks like: `Hooks<impl
/// CommonHooks, impl MasterHooks, impl SlaveHooks, impl RgbHooks>`.
/// And this trait is implemented for `Hooks<CH, MH, SH, RH>` so users can just write `impl
/// AllHooks`.
///
/// Though this trait is not sealed, it is not recommended to implement this trait for your own
/// types.
pub trait AllHooks {
    type Common: CommonHooks;
    type Master: MasterHooks;
    type Slave: SlaveHooks;
    type Rgb: RgbHooks;

    fn destructure(self) -> Hooks<Self::Common, Self::Master, Self::Slave, Self::Rgb>;
}

impl<CH: CommonHooks, MH: MasterHooks, SH: SlaveHooks, RH: RgbHooks> AllHooks
    for Hooks<CH, MH, SH, RH>
{
    type Common = CH;
    type Master = MH;
    type Slave = SH;
    type Rgb = RH;

    fn destructure(self) -> Hooks<CH, MH, SH, RH> {
        self
    }
}

/// Collection of sender/receiver that can be used with hooks.
pub mod channels {
    pub use crate::task::channels::*;
}

/// Collection of empty hooks and utility functions.
pub mod empty_hooks {
    use crate::hooks::AllHooks;

    use super::{
        Hooks, MasterHooks, SlaveHooks,
        interface::{CommonHooks, RgbHooks},
    };

    pub struct EmptyCommonHooks;
    impl CommonHooks for EmptyCommonHooks {}

    pub struct EmptyMasterHooks;
    impl MasterHooks for EmptyMasterHooks {}

    pub struct EmptySlaveHooks;
    impl SlaveHooks for EmptySlaveHooks {}

    pub struct EmptyRgbHooks;
    impl RgbHooks for EmptyRgbHooks {}

    pub const fn create_empty_hooks() -> impl AllHooks {
        Hooks {
            common: EmptyCommonHooks,
            master: EmptyMasterHooks,
            slave: EmptySlaveHooks,
            rgb: EmptyRgbHooks,
        }
    }
}
