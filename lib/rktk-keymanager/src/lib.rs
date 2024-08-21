#![cfg_attr(all(not(test), not(feature = "specta")), no_std)]

//! # rktk-keymanager
//! A library for managing key state and keymaps for self-made keyboards.

use keycode::KeyAction;

pub mod keycode;
#[cfg(any(test, feature = "state"))]
pub mod state;
#[cfg(not(any(test, feature = "state")))]
pub mod state {
    pub mod config;
}

#[derive(Clone, Debug)]
pub struct Layer<const ROW: usize, const COL: usize> {
    pub map: [[KeyAction; COL]; ROW],
    pub arrowmouse: bool,
}
pub type LayerMap<const ROW: usize, const COL: usize> = [[KeyAction; COL]; ROW];
pub type Keymap<const LAYER: usize, const ROW: usize, const COL: usize> = [Layer<ROW, COL>; LAYER];

pub mod time {
    use core::ops::{Add, Sub};
    pub use core::time::Duration;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Instant {
        from_start: core::time::Duration,
    }

    impl Instant {
        pub const fn from_start(from_start: core::time::Duration) -> Self {
            Self { from_start }
        }
    }

    impl Add<Duration> for Instant {
        type Output = Self;
        fn add(self, rhs: Duration) -> Self {
            Self {
                from_start: self.from_start + rhs,
            }
        }
    }

    impl Sub<Instant> for Instant {
        type Output = Duration;
        fn sub(self, rhs: Instant) -> Duration {
            self.from_start - rhs.from_start
        }
    }
}
