use clap::Subcommand;

mod check;
mod config;
mod doc;
mod release;
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
    Release {
        crate_name: Option<String>,

        #[arg(long)]
        execute: bool,
        #[arg(long)]
        continue_on_error: bool,
    },
    Doc,
}

pub fn start(command: InternalCommands) -> anyhow::Result<()> {
    match command {
        InternalCommands::Check { crate_name } => check::start(crate_name)?,
        InternalCommands::Test { crate_name } => test::start(crate_name)?,
        InternalCommands::Release {
            crate_name,
            execute,
            continue_on_error,
        } => release::start(crate_name, execute, continue_on_error)?,
        InternalCommands::Doc => doc::start()?,
    };

    Ok(())
}
