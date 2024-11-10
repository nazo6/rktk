use crate::state::config::Output;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransparentReport {
    pub flash_clear: bool,
    pub output: Output,
}

impl TransparentReport {
    pub const fn new(initial_output: Output) -> Self {
        Self {
            flash_clear: false,
            output: initial_output,
        }
    }
}
