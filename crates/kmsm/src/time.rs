use core::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    /// Time from start in milliseconds.
    from_start: u32,
}

impl Instant {
    #[allow(dead_code)]
    pub const fn from_start(from_start: Duration) -> Self {
        Self {
            from_start: from_start.millis,
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self {
        Self {
            from_start: self.from_start + rhs.millis,
        }
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Duration {
        Duration {
            millis: self.from_start - rhs.from_start,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    millis: u32,
}

impl Duration {
    #[allow(dead_code)]
    pub const fn from_millis(millis: u32) -> Self {
        Self { millis }
    }
}

impl From<core::time::Duration> for Duration {
    fn from(d: core::time::Duration) -> Self {
        Self {
            millis: d.as_millis() as u32,
        }
    }
}
