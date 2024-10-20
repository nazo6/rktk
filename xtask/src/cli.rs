use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Build keyboard binary in specified path
    Build(build::BuildCommand),
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
    /// Intended to be used from rust-analyzer to provide per-crate `cargo clippy` diagnostics.
    RaCheck { saved_file: String },
}

pub mod build {
    use clap::{Args, ValueEnum};

    #[derive(Debug, Args)]
    pub struct BuildCommand {
        pub path: String,
        #[arg(value_enum)]
        pub mcu: BuildMcu,
        /// Deploy the binary to the specified path
        /// If this is specified, `uf2` will be ignored and always set to true.
        #[arg(long, short)]
        pub deploy_dir: Option<String>,
        /// Profile to use for building the binary.
        /// This internally use cargo profile, but they are different things.
        #[arg(long, short, default_value_t = BuildProfile::MinSize, value_enum)]
        pub profile: BuildProfile,
        /// Convert the binary to uf2 format.
        #[arg(long, default_value_t = true)]
        pub uf2: bool,
        /// Retry count for deploying the binary.
        #[arg(long, default_value_t = 40)]
        pub deploy_retry_count: u32,

        /// Additional options for `cargo build`.
        #[arg(last = true)]
        pub cargo_build_opts: Vec<String>,
    }

    #[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
    pub enum BuildProfile {
        MinSize,
        MaxPerf,
    }

    #[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
    pub enum BuildMcu {
        Rp2040,
        Nrf52840,
    }
}
