use crate::state::config::Output;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransparentReport {
    pub flash_clear: bool,
    pub ble_bond_clear: bool,
    pub output: Output,
}

impl TransparentReport {
    pub const fn new() -> Self {
        Self {
            flash_clear: false,
            ble_bond_clear: false,
            output: Output::Usb,
        }
    }
}
