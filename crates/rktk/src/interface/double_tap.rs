use embassy_time::Duration;

pub trait DoubleTapReset {
    async fn execute(&self, timeout: Duration);
}
