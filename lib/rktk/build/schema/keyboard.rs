use const_gen::CompileConst;

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
    pub right_led_count: usize,

    /// RGB led count for left side. This is also used for non-split keyboard.
    #[serde(default)]
    pub left_led_count: usize,
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
