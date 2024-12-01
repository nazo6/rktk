use super::reporter::ReporterDriver;

pub trait BleDriver: ReporterDriver {
    type Error: core::error::Error;

    async fn clear_bond_data(&self) -> Result<(), <Self as BleDriver>::Error>;
}
