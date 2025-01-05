use macro_rules_attribute::apply;

use crate::macros::common_derive;

#[cfg(feature = "state")]
pub mod report;
pub mod state;

#[apply(common_derive)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Copy)]
pub enum Output {
    Usb,
    Ble,
}
