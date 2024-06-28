use konst::{primitive::parse_usize, unwrap_ctx};

pub const COL_PIN_COUNT: usize = unwrap_ctx!(parse_usize(env!("RKTK_COL_PIN_COUNT")));
pub const ROW_PIN_COUNT: usize = unwrap_ctx!(parse_usize(env!("RKTK_ROW_PIN_COUNT")));

/// 片手の列数
pub const COLS: usize = unwrap_ctx!(parse_usize(env!("RKTK_COLS")));
/// 片手の行数
pub const ROWS: usize = unwrap_ctx!(parse_usize(env!("RKTK_ROWS")));

pub const LAYER_COUNT: usize = unwrap_ctx!(parse_usize(env!("RKTK_LAYER_COUNT")));
