use crate::state::config::Output;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransparentReport {
    pub flash_clear: bool,
    pub ble_bond_clear: bool,
    pub output: Output,
    pub bootloader: bool,
}
