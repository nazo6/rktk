use rktk_keymanager::state::config::{
    KeyResolverConfig, MouseConfig, TapDanceConfig, TapHoldConfig,
};

#[cfg_attr(
    not(no_build),
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Default)
)]
pub struct StaticConfig {
    pub keyboard: Keyboard,
    pub config: Config,
}

// layout is json data in config file. but in keyboard, it's just a string.
#[cfg(not(no_build))]
fn serialize_layout<S: serde::Serializer>(
    val: &serde_json::Value,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&serde_json::to_string(val).unwrap())
}

#[cfg(not(no_build))]
type NameType = String;
#[cfg(no_build)]
type NameType = &'static str;

#[cfg(not(no_build))]
type LayoutType = serde_json::Value;
#[cfg(no_build)]
type LayoutType = &'static str;

/// Keyboard layout and informations.
///
/// This struct is used to
/// - Defines keyboard basic informations (ex: name, cols, rows, ...)
/// - Defines keyboard physical layout which is used by remapper (layout property)
///
/// # Coordination of the keyboard matrix
///
/// The rktk coordinate system has the top left as (0,0), and the coordinate values increase toward the bottom right.
///
/// ## Split keyboard coordinates
/// For `col` in keyboard config, specify the coordinates of the entire keyboard.
/// In other words, for a split keyboard with 7 columns on the left hand side and 7 columns on the right hand side, specify 14.
///
/// Internally, the key scan driver returns the coordinates of "only one hand." In other words, in this case, x=0-6.
/// Therefore, it is necessary to convert the coordinates received from the key scan driver into the coordinates of both hands,
/// and for this purpose the `split_right_shift` property is used.
///
/// Below is an example of a split keyboard with 14 columns and 4 rows.
/// ```ignored
///            [    Left    ]   [     Right     ]
///            0 1 2 3 4 5 6    0 1 2  3  4  5  6 ← One-handed coordinates
///                             ↓ split_right_shift=7 (or None)
/// col=14 →   0 1 2 3 4 5 6    7 8 9 10 11 12 13 ← Two-handed coordinates
///          0 _ Q W E R T _    _ Y U  I  O  P  _
///          1 ...
///          2 ...
///          3 ...
///          ↑ row=4
/// ```
#[cfg_attr(
    not(no_build),
    derive(
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        smart_default::SmartDefault
    )
)]
pub struct Keyboard {
    /// The name of the keyboard.
    pub name: NameType,

    /// Defines the layout of the keyboard used in the remapper.
    ///
    /// This is a JSON object that represents the layout of the keyboard and compatible with via's
    /// json layout format.
    #[cfg_attr(not(no_build), serde(serialize_with = "serialize_layout"))]
    pub layout: LayoutType,

    /// The number of columns in the keyboard matrix.
    pub cols: u8,

    /// The number of rows in the keyboard matrix.
    pub rows: u8,

    /// A number representing the row number that the right col starts on in a split keyboard.
    ///
    /// If not set, `cols / 2` will be automatically set,
    /// so there is no need to set it if the number of columns on the right and left sides is the same.
    /// Also, there is no need to set it in the case of a non-split keyboard, as it is not used.
    #[cfg_attr(not(no_build), serde(default))]
    pub split_right_shift: Option<u8>,

    /// The number of encoder keys.
    #[cfg_attr(not(no_build), serde(default))]
    #[cfg_attr(not(no_build), default(0))]
    pub encoder_count: u8,

    /// RGB led count for right side
    #[cfg_attr(not(no_build), serde(default))]
    #[cfg_attr(not(no_build), default(0))]
    pub right_led_count: usize,

    /// RGB led count for left side. This is also used for non-split keyboard.
    #[cfg_attr(not(no_build), serde(default))]
    #[cfg_attr(not(no_build), default(0))]
    pub left_led_count: usize,
}

/// Configuration for the firmware.
#[cfg_attr(
    not(no_build),
    derive(
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        smart_default::SmartDefault
    )
)]
#[cfg_attr(not(no_build), serde(default))]
pub struct Config {
    pub rktk: RktkConfig,
    pub key_manager: KeyManagerConfig,
}

/// RKTK behavior configuration.
///
/// Mainly keymap related configurations are defined in this struct.
#[cfg_attr(
    not(no_build),
    derive(
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        smart_default::SmartDefault
    )
)]
#[cfg_attr(not(no_build), serde(default))]
pub struct RktkConfig {
    /// The number of layers in the keyboard.
    #[cfg_attr(not(no_build), default(5))]
    pub layer_count: u8,

    /// Threshold for double tap (ms).
    #[cfg_attr(not(no_build), default(500))]
    pub double_tap_threshold: u64,

    /// Default CPI value for mouse
    #[cfg_attr(not(no_build), default(600))]
    pub default_cpi: u16,

    /// Default duration of auto mouse mode (ms)
    #[cfg_attr(not(no_build), default(500))]
    pub default_auto_mouse_duration: u32,

    /// Timeout for detecting split USB connection (ms).
    #[cfg_attr(not(no_build), default(1000))]
    pub split_usb_timeout: u64,

    /// Time (ms) to wait for the next keyboard scan
    #[cfg_attr(not(no_build), default(5))]
    pub scan_interval_keyboard: u64,

    /// Time (ms) to wait for the next mouse scan
    #[cfg_attr(not(no_build), default(5))]
    pub scan_interval_mouse: u64,

    /// The size of the split channel. Usually, you don't need to change this value.
    #[cfg_attr(not(no_build), default(64))]
    pub split_channel_size: usize,
}

/// Configs passed to [`rktk_keymanager`]
#[cfg_attr(
    not(no_build),
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
#[cfg_attr(not(no_build), serde(default))]
pub struct KeyManagerConfig {
    pub mouse: MouseConfig,
    pub key_resolver: KeyResolverConfig,
}

impl Default for KeyManagerConfig {
    fn default() -> Self {
        Self {
            mouse: MouseConfig {
                auto_mouse_layer: 1,
                auto_mouse_duration: 500,
                auto_mouse_threshold: 1,
                scroll_divider_x: 20,
                scroll_divider_y: 12,
            },
            key_resolver: KeyResolverConfig {
                tap_hold: TapHoldConfig {
                    threshold: 500,
                    hold_on_other_key: true,
                },
                tap_dance: TapDanceConfig { threshold: 100 },
            },
        }
    }
}
