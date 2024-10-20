use clap::Parser as _;
use cli::Cli;
use utils::xprintln;

mod cli;
mod commands;
mod utils;

fn main() {
    let args = Cli::parse();
    let res = match args.command {
        cli::Commands::Build(build_command) => commands::build::start(build_command),
        cli::Commands::Check { crate_path } => commands::check::start(crate_path),
        cli::Commands::RaCheck { saved_file } => commands::ra_check::start(saved_file),
    };

    eprintln!();

    match res {
        Ok(_) => {
            xprintln!("{}", "SUCCESS".green());
        }
        Err(err) => {
            xprintln!("{} {}", " ERROR ".on_red(), err.to_string().red());
        }
    }
}
