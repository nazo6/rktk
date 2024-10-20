macro_rules! xprintln {
    ($($arg:tt)*) => {{
        use colored::*;

        eprint!("{} ", " xtask ".on_blue(),);
        eprintln!($($arg)*);
    }};
}
pub(crate) use xprintln;
