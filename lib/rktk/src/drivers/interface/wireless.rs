//! Bluetooth Low Energy (BLE) driver type.

use super::reporter::ReporterDriver;

/// BLE driver type.
pub trait WirelessReporterDriver: ReporterDriver {
    type Error: super::Error;

    /// Clears all bond data
    async fn clear_bond_data(&self) -> Result<(), <Self as WirelessReporterDriver>::Error>;
}

super::generate_builder!(WirelessReporterDriver);
