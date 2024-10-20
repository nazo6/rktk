mod mcu;
mod profile;

use anyhow::Context as _;
use profile::PROFILE_CONFIG_TOML;
use std::io::Write as _;

use crate::{
    cli::build::{BuildCommand, BuildMcu, BuildProfile},
    utils::xprintln,
};

pub fn start(args: BuildCommand) -> anyhow::Result<()> {
    let crate_path = std::path::PathBuf::from(&args.path).canonicalize()?;

    let mut config_toml_file =
        tempfile::NamedTempFile::new().context("Failed to create a temporary file")?;
    write!(
        config_toml_file,
        "{}",
        toml::to_string_pretty(&*PROFILE_CONFIG_TOML).unwrap()
    )
    .context("Failed to write to the temporary file")?;

    let mcu_config = match args.mcu {
        BuildMcu::Rp2040 => mcu::MCU_CONFIG_RP2040,
        BuildMcu::Nrf52840 => mcu::MCU_CONFIG_NRF52840,
    };
    let profile = match args.profile {
        BuildProfile::MinSize => &profile::PROFILE_MIN_SIZE,
        BuildProfile::MaxPerf => &profile::PROFILE_MAX_PERF,
    };

    let mut cmd_args = vec![
        "--config".to_string(),
        config_toml_file.path().to_string_lossy().to_string(),
        "build".to_string(),
        "--profile".to_string(),
        profile.name.to_string(),
        "--target".to_string(),
        mcu_config.target.to_string(),
    ];

    if let Some(build_std) = &profile.cargo_cmd.build_std {
        cmd_args.push("-Z".to_string());
        cmd_args.push(format!("build-std={}", build_std));
    }
    if let Some(build_std_features) = &profile.cargo_cmd.build_std_features {
        cmd_args.push("-Z".to_string());
        cmd_args.push(format!("build-std-features={}", build_std_features));
    }

    xprintln!(
        "Building with profile `{:?}`, target `{:?}` (args: {:?})",
        profile.name,
        args.mcu,
        cmd_args
    );

    let cmd = duct::cmd("cargo", cmd_args).dir(crate_path);

    cmd.run()?;

    Ok(())
}
