//! Key scanning driver implementations.

use rktk::drivers::interface::keyscan::Hand;

pub mod duplex_matrix;
pub mod flex_pin;
pub mod matrix;
mod pressed;
pub mod shift_register_matrix;

/// Way to detect hand.
pub enum HandDetector {
    /// Detect by key position. (col, row)
    ///
    /// This is useful for keyboard that has jumper to detect hand.
    ByKey(usize, usize),
    /// Use provided constant value as hand.
    ///
    /// This is useful for keyboard that has no system to detect hand.
    /// This is also useful for non-split keyboard. In non-split keyboard, use variant and provide `Hand::Left`.
    Constant(Hand),
}
