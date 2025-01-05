use const_gen::CompileConst;

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
#[macro_rules_attribute::apply(crate::schema::common_derive)]
pub struct Keyboard {
    /// The name of the keyboard.
    pub name: String,

    /// Defines the layout of the keyboard used in the remapper.
    ///
    /// This is a JSON object that represents the layout of the keyboard and compatible with via's
    /// json layout format.
    pub layout: Option<KeyboardLayout>,

    /// The number of columns in the keyboard matrix.
    pub cols: u8,

    /// The number of rows in the keyboard matrix.
    pub rows: u8,

    /// A number representing the row number that the right col starts on in a split keyboard.
    ///
    /// If not set, `cols / 2` will be automatically set,
    /// so there is no need to set it if the number of columns on the right and left sides is the same.
    /// Also, there is no need to set it in the case of a non-split keyboard, as it is not used.
    #[serde(default)]
    pub split_right_shift: Option<u8>,

    /// The number of encoder keys.
    #[serde(default)]
    pub encoder_count: u8,

    /// RGB led count for right side
    #[serde(default)]
    pub right_rgb_count: usize,

    /// RGB led count for left side. This is also used for non-split keyboard.
    #[serde(default)]
    pub left_rgb_count: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct KeyboardLayout(serde_json::Value);

impl CompileConst for KeyboardLayout {
    fn const_type() -> String {
        "&'static str".to_string()
    }

    fn const_val(&self) -> String {
        format!(
            "r######\"{}\"######",
            serde_json::to_string(&self.0).unwrap()
        )
    }
}
