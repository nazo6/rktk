use macro_rules_attribute::apply;

use crate::macros::{common_derive, with_consts};

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
}
