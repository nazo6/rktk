use embassy_time::Duration;

pub trait DoubleTapReset {
    async fn execute(&self, timeout: Duration);
}

pub struct DummyDoubleTapReset;
impl DoubleTapReset for DummyDoubleTapReset {
    async fn execute(&self, _timeout: Duration) {}
}
