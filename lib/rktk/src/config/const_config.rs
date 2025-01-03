use konst::{
    option::{map, unwrap_or},
    primitive::{parse_u8, parse_usize},
    unwrap_ctx,
};

/// Config that needs to be known at compile time.
///
/// Thse values are read from environment variables set at build time.
///
/// Also see [`rktk_keymanager::config::ConstConfig`] for keymap related const config.
pub struct ConstConfig {
    /// Column count of the keyboard matrix.
    ///
    /// env: `RKTK_KEYBOARD_COLS`
    pub cols: u8,

    /// Row count of the keyboard matrix.
    ///
    /// env: `RKTK_KEYBOARD_ROWS`
    pub rows: u8,

    /// Layer count of the keymap.
    ///
    /// env: `RKTK_KEYBOARD_LAYER_COUNT`
    pub layer_count: u8,

    /// Encoder count of the keyboard.
    ///
    /// env: `RKTK_KEYBOARD_ENCODER_COUNT`
    pub encoder_count: u8,

    /// A number representing the row number that the right col starts on in a split keyboard.
    ///
    /// If not set, `cols / 2` will be automatically set,
    /// so there is no need to set it if the number of columns on the right and left sides is the same.
    /// Also, there is no need to set it in the case of a non-split keyboard, as it is not used.
    pub split_right_shift: Option<u8>,

    /// RGB led count for right side
    pub right_led_count: usize,

    /// RGB led count for left side. This is also used for non-split keyboard.
    pub left_led_count: usize,

    pub split_channel_size: usize,
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
