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
        /// (relative) path to the crate to check
        /// If 'all' is specified, all crates will be checked.
        crate_path: String,
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
        pub deploy: Option<String>,
        /// Profile to use for building the binary.
        /// This internally use cargo profile, but they are different things.
        #[arg(long, default_value_t = BuildProfile::MaxPerf, value_enum)]
        pub profile: BuildProfile,
        /// Convert the binary to uf2 format.
        #[arg(long, default_value_t = true)]
        pub uf2: bool,
    }

    #[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
    pub enum BuildProfile {
        MinSize,
        MaxPerf,
    }

    impl ToString for BuildProfile {
        fn to_string(&self) -> String {
            match self {
                Self::MinSize => "min-size".to_string(),
                Self::MaxPerf => "max-perf".to_string(),
            }
        }
    }

    #[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
    pub enum BuildMcu {
        Rp2040,
        Nrf52840,
    }
}
