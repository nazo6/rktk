//! Keyboard-specific configs.
//!
//! This can be customized through the environment variables, so each keyboard crate can change
//! this using `.cargo/config.toml`.

use konst::{primitive::parse_usize, unwrap_ctx};

/// The number of pins used for column.
/// Env key: `RKTK_COL_PIN_COUNT`
pub const COL_PIN_COUNT: usize = unwrap_ctx!(parse_usize(env!("RKTK_COL_PIN_COUNT")));
/// The number of pins used for row.
/// Env key: `RKTK_ROW_PIN_COUNT`
pub const ROW_PIN_COUNT: usize = unwrap_ctx!(parse_usize(env!("RKTK_ROW_PIN_COUNT")));

/// The number of columns in the keyboard matrix.
/// Env key: `RKTK_COLS`
pub const COLS: usize = unwrap_ctx!(parse_usize(env!("RKTK_COLS")));
/// The number of rows in the keyboard matrix.
/// Env key: `RKTK_ROWS`
pub const ROWS: usize = unwrap_ctx!(parse_usize(env!("RKTK_ROWS")));

/// The number of layers in the keyboard.
/// Env key: `RKTK_LAYER_COUNT`
///
/// Making this value larger may cause memory overflow.
pub const LAYER_COUNT: usize = unwrap_ctx!(parse_usize(env!("RKTK_LAYER_COUNT")));
