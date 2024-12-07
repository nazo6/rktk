//! Special keys.

use macro_rules_attribute::apply;

use crate::macros::{common_derive, with_consts};

/// Represents special keys.
///
/// "Special Key" is a key that is not intended to be sent externally. There are two types of keys in this category:
/// 1. "Transparent" keys
///    These keys have no meaning in this crate. If these keys are pressed, its information are
///    passed through transparent report to caller.
/// 2. Keys that determine the behavior of rktk-keymanager
///    These keys are used to control the behavior of rktk-keymanager.
///    For example, while `MoScrl` key is pressed, mouse event is converted to scroll event.
#[apply(with_consts)]
#[apply(common_derive)]
#[derive(Copy)]
pub enum Special {
    MoScrl,
    AmlReset,
    FlashClear,
    OutputBle,
    OutputUsb,
    BleBondClear,
    Bootloader,
    PowerOff,
}
