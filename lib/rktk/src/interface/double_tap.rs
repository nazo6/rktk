use embassy_time::Duration;

pub trait DoubleTapResetDriver {
    async fn execute(&self, timeout: Duration);
}

pub struct DummyDoubleTapResetDriver;
impl DoubleTapResetDriver for DummyDoubleTapResetDriver {
    async fn execute(&self, _timeout: Duration) {}
}
