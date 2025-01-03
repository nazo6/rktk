use konst::{
    option::{map, unwrap_or},
    primitive::{parse_u8, parse_usize},
    unwrap_ctx,
};

/// Config that needs to be known at compile time.
///
/// Thse values are read from environment variables set at build time.
pub struct ConstConfig {
    pub cols: u8,
    pub rows: u8,
    pub layer_count: u8,
    pub encoder_count: u8,
    pub split_right_shift: Option<u8>,

    pub split_channel_size: usize,

    pub right_led_count: usize,
    pub left_led_count: usize,
}

pub const CONST_CONFIG: ConstConfig = ConstConfig {
    cols: unwrap_ctx!(parse_u8(env!("RKTK_KEYBOARD_COLS"))),
    rows: unwrap_ctx!(parse_u8(env!("RKTK_KEYBOARD_ROWS"))),
    layer_count: unwrap_ctx!(parse_u8(unwrap_or!(
        option_env!("RKTK_KEYBOARD_LAYER_COUNT"),
        "5"
    ))),
    encoder_count: unwrap_ctx!(parse_u8(unwrap_or!(
        option_env!("RKTK_KEYBOARD_ENCODER_COUNT"),
        "0"
    ))),
    split_right_shift: map!(
        option_env!("RKTK_KEYBOARD_SPLIT_RIGHT_SHIFT"),
        |x| unwrap_ctx!(parse_u8(x))
    ),

    split_channel_size: unwrap_ctx!(parse_usize(unwrap_or!(
        option_env!("RKTK_BUFFER_SIZE_SPLIT_CHANNEL"),
        "10"
    ))),

    right_led_count: unwrap_ctx!(parse_usize(unwrap_or!(
        option_env!("RKTK_RIGHT_LED_COUNT"),
        "0"
    ))),
    left_led_count: unwrap_ctx!(parse_usize(unwrap_or!(
        option_env!("RKTK_LEFT_LED_COUNT"),
        "0"
    ))),
};
