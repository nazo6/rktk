use super::macros::with_consts_no_val;

with_consts_no_val!(
    Special,
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum Special {
        MoScrl,
    }
);
