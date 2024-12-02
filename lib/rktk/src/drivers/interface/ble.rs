//! Bluetooth Low Energy (BLE) driver type.

use super::reporter::ReporterDriver;

/// BLE driver type.
pub trait BleDriver: ReporterDriver {
    type Error: core::error::Error;

    /// Clears all bond data
    async fn clear_bond_data(&self) -> Result<(), <Self as BleDriver>::Error>;
}
