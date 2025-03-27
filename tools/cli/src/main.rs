use std::process::ExitCode;

use utils::xprintln;

mod check;
mod config;
mod doc;
mod release;
mod test;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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

fn main() -> ExitCode {
    let args = Cli::parse();
    let res = match args.command {
        Commands::Check { crate_name } => check::start(crate_name),
        Commands::Test { crate_name } => test::start(crate_name),
        Commands::Release {
            crate_name,
            execute,
            continue_on_error,
        } => release::start(crate_name, execute, continue_on_error),
        Commands::Doc => doc::start(),
    };

    eprintln!();

    match res {
        Ok(_) => {
            xprintln!("{}", "SUCCESS".green());
            ExitCode::from(0)
        }
        Err(err) => {
            xprintln!("{} {}", " ERROR ".on_red(), format!("{:?}", err).red());
            ExitCode::from(1)
        }
    }
}
