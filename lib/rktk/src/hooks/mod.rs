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

/// Collection of sender/receiver that can be used with hooks.
pub mod channels {
    pub use crate::task::channels::{
        report::{encoder_event_sender, keyboard_event_sender, mouse_event_sender},
        rgb::rgb_sender,
    };
}

/// Collection of empty hooks and utility functions.
pub mod empty_hooks {
    use super::{
        interface::{CommonHooks, RgbHooks},
        Hooks, MasterHooks, SlaveHooks,
    };

    pub struct EmptyCommonHooks;
    impl CommonHooks for EmptyCommonHooks {}

    pub struct EmptyMasterHooks;
    impl MasterHooks for EmptyMasterHooks {}

    pub struct EmptySlaveHooks;
    impl SlaveHooks for EmptySlaveHooks {}

    pub struct EmptyRgbHooks;
    impl RgbHooks for EmptyRgbHooks {}

    pub const fn create_empty_hooks(
    ) -> Hooks<EmptyCommonHooks, EmptyMasterHooks, EmptySlaveHooks, EmptyRgbHooks> {
        Hooks {
            common: EmptyCommonHooks,
            master: EmptyMasterHooks,
            slave: EmptySlaveHooks,
            rgb: EmptyRgbHooks,
        }
    }
}
