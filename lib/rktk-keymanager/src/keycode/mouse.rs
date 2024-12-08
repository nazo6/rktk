use macro_rules_attribute::apply;

use crate::macros::with_consts;

use super::common_derive;

#[apply(with_consts)]
#[apply(common_derive)]
#[derive(Copy, strum::EnumIter, strum::IntoStaticStr)]
pub enum Mouse {
    Left = 0b0000_0001,
    Right = 0b0000_0010,
    Middle = 0b0000_0100,
    Back = 0b0000_1000,
    Forward = 0b0001_0000,
}
