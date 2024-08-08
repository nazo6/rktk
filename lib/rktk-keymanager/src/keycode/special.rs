use super::macros::with_consts_no_val;

with_consts_no_val!(
    Special,
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    /// Special key definitions.
    ///
    /// - `MoScrl`: Enable mouse scroll mode when held.
    pub enum Special {
        MoScrl,
    }
);
