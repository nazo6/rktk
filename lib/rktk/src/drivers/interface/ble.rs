use super::reporter::ReporterDriver;

pub trait BleDriver: ReporterDriver {
    async fn clear_bond_data(&self) {}
}
