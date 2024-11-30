use core::ops::{Add, Sub};
pub use core::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    /// Time from start in milliseconds.
    from_start: u16,
}

impl Instant {
    pub const fn from_start(from_start: core::time::Duration) -> Self {
        Self {
            from_start: from_start.as_millis() as u16,
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self {
        Self {
            from_start: self.from_start + rhs.as_millis() as u16,
        }
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Duration {
        Duration::from_millis(self.from_start as u64 - rhs.from_start as u64)
    }
}
