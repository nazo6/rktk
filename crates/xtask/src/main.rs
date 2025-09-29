use std::process::ExitCode;

use utils::xprintln;

mod check;
mod config;
mod doc;
mod publish;
mod stats;
mod test;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone)]
pub enum CrateFilter {
    /// Choose all crates.
    All,
    /// Choose only binary crates (`check_build` is false, usually in `/crates`).
    Lib,
    /// Choose only binary crates (`check_build` is true, usually in `/keyboards`).
    Bin,
    /// Choose only specified crate.
    CrateName(String),
}

impl std::str::FromStr for CrateFilter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(CrateFilter::All),
            "lib" => Ok(CrateFilter::Lib),
            "bin" => Ok(CrateFilter::Bin),
            other => Ok(CrateFilter::CrateName(other.to_string())),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run `cargo clippy` with feature matrix for rktk crates.
    Check { crate_filter: CrateFilter },
    /// Run `cargo test` for rktk crates.
    Test {
        /// crate name to run test
        /// If 'all' is specified, all crates will be tested.
        crate_name: String,
    },
    /// Publish crates
    Publish {
        crate_name: Option<String>,
        /// By default, `cargo publish` will be executed with `--dry-run`.
        /// If true, `cargo publish` will be executed without `--dry-run`.
        #[arg(long)]
        execute: bool,
        // If true, continue publishing other crates even if one crate fails to publish.
        #[arg(long)]
        continue_on_error: bool,
    },
    /// Generate documentation for rktk crates.
    Doc,
    Stats {
        /// crate name to show stats. This must be binary crate.
        crate_name: String,
        /// Specify binary name if the crate has multiple binaries.
        /// If not specified, the first binary will be used.
        #[arg(long)]
        bin: String,
        #[arg(long)]
        features: Vec<String>,
        #[arg(long)]
        /// If true, output will be written to `/tmp/stats-comment.md` for GitHub Actions.
        gh_output: bool,
        #[arg(long)]
        /// If false, only show stats about binary size. If true, it performs extra analysis and shows more detailed stats.
        extra: bool,
    },
}

fn main() -> ExitCode {
    let args = Cli::parse();
    let res = match args.command {
        Commands::Check { crate_filter } => check::start(crate_filter),
        Commands::Test { crate_name } => test::start(crate_name),
        Commands::Publish {
            crate_name,
            execute,
            continue_on_error,
        } => publish::start(crate_name, execute, continue_on_error),
        Commands::Doc => doc::start(),
        Commands::Stats {
            crate_name,
            bin,
            features,
            gh_output,
            extra,
        } => stats::start(crate_name, bin, features, gh_output, extra),
    };

    eprintln!();

    match res {
        Ok(_) => {
            xprintln!("{}", "SUCCESS".green());
            ExitCode::from(0)
        }
        Err(err) => {
            xprintln!("{} {}", " ERROR ".on_red(), format!("{err:?}").red());
            ExitCode::from(1)
        }
    }
}
