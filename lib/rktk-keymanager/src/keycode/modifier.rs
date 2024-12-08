//! Modifier keys

use macro_rules_attribute::apply;

use crate::macros::with_consts;

use super::common_derive;

#[apply(with_consts)]
#[apply(common_derive)]
#[derive(Copy, strum::EnumIter, strum::IntoStaticStr)]
pub enum Modifier {
    LCtrl = 0x01,
    LShft = 0x02,
    LAlt = 0x04,
    LGui = 0x08,
    RCtrl = 0x10,
    RShft = 0x20,
    RAlt = 0x40,
    RGui = 0x80,
}
