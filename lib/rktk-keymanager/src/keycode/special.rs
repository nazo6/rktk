use super::macros::with_consts_no_val;

with_consts_no_val!(
    Special,
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(
        feature = "postcard",
        derive(postcard::experimental::max_size::MaxSize)
    )]
    #[cfg_attr(feature = "specta", derive(specta::Type))]
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    /// Special key definitions.
    ///
    /// - `MoScrl`: Enable mouse scroll mode when held.
    pub enum Special {
        MoScrl,
    }
);
