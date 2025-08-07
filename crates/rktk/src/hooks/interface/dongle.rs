use crate::drivers::interface::dongle::DongleData;

pub trait DongleHooks {
    async fn on_dongle_data(&mut self, _data: &mut DongleData) -> bool {
        true
    }
}

pub struct DummyDongleHooks {}
impl DongleHooks for DummyDongleHooks {}

pub fn default_dongle_hooks() -> impl DongleHooks {
    DummyDongleHooks {}
}
