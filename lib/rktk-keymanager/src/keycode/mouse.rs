use macro_rules_attribute::apply;

use crate::macros::{impl_display, with_consts};

use super::common_derive;

#[apply(with_consts)]
#[apply(common_derive)]
#[derive(Copy, strum::EnumIter, strum::IntoStaticStr)]
pub enum Mouse {
    MLeft = 0b0000_0001,
    MRight = 0b0000_0010,
    MMiddle = 0b0000_0100,
    MBack = 0b0000_1000,
    MForward = 0b0001_0000,
}

impl_display!(Mouse);
