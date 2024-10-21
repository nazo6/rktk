use clap::Subcommand;

mod check;
mod test;

/// Internal commands for rktk repo.
#[derive(Debug, Subcommand)]
pub enum InternalCommands {
    /// Run `cargo clippy` for rktk crates.
    Check {
        /// crate name to check
        /// If 'all' is specified, all crates will be checked.
        crate_name: String,
    },
    Test {
        /// crate name to run test
        /// If 'all' is specified, all crates will be tested.
        crate_name: String,
    },
}

pub fn start(command: InternalCommands) -> anyhow::Result<()> {
    match command {
        InternalCommands::Check { crate_name } => check::start(crate_name)?,
        InternalCommands::Test { crate_name } => test::start(crate_name)?,
    };

    Ok(())
}
