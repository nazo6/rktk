use macro_rules_attribute::apply;

use crate::macros::common_derive;

#[cfg(feature = "state")]
pub mod report;
pub mod state;

#[apply(common_derive)]
#[derive(Copy)]
pub enum Output {
    Usb,
    Ble,
}
