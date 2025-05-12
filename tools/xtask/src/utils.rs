macro_rules! xprintln {
    ($($arg:tt)*) => {{
        use colored::*;

        eprint!("{} ", " rktk ".on_blue(),);
        eprintln!($($arg)*);
    }};
}
use std::sync::LazyLock;

use cargo_metadata::Metadata;
pub(crate) use xprintln;

/// Metadata for the current (dir) workspace.
pub(crate) static METADATA: LazyLock<Option<Metadata>> =
    LazyLock::new(|| cargo_metadata::MetadataCommand::new().no_deps().exec().ok());
