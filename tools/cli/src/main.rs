use utils::xprintln;

mod commands;
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
    /// Builds keyboard
    Build(commands::build::BuildCommand),
    #[command(subcommand)]
    Internal(commands::internal::InternalCommands),
}

fn main() {
    let args = Cli::parse();
    let res = match args.command {
        Commands::Build(build_args) => commands::build::start(build_args),
        Commands::Internal(internal_command) => commands::internal::start(internal_command),
    };

    eprintln!();

    match res {
        Ok(_) => {
            xprintln!("{}", "SUCCESS".green());
        }
        Err(err) => {
            xprintln!("{} {}", " ERROR ".on_red(), format!("{:?}", err).red());
        }
    }
}
