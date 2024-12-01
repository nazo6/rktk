mod config;
mod mcu;
mod profile;
mod uf2;

use anyhow::Context as _;
use colored::Colorize as _;
use config::{BuildConfig, BuildMcu};
use profile::{BuildProfileList, PROFILE_CONFIG_TOML};
use std::{io::Write as _, path::PathBuf};

use crate::utils::{xprintln, METADATA};

use clap::Args;

#[derive(Debug, Args)]
pub struct BuildCommand {
    /// Path of the keyboard crate to build. If not specified, the current directory will be used.
    #[arg(default_value_t = {".".to_string()})]
    pub path: String,

    /// Chip mcu.
    /// Overrides value in `rktk.build.json`
    #[arg(long, short, value_enum, verbatim_doc_comment)]
    pub mcu: Option<BuildMcu>,

    /// Profile to use for building the binary. This internally use cargo profile, but they are different things.
    /// If not specified, `min-size` will be used.
    /// Overrides value in `rktk.build.json`
    #[arg(long, short, value_enum, verbatim_doc_comment)]
    pub profile: Option<BuildProfileList>,

    /// Deploy the binary to the specified path
    /// If this is specified, `--no-uf2` will be ignored.
    #[arg(long, short, verbatim_doc_comment)]
    pub deploy_dir: Option<String>,
    /// Doesn't generate uf2 file.
    #[arg(long)]
    pub no_uf2: bool,
    /// Retry count for deploying the binary.
    #[arg(long, default_value_t = 40)]
    pub deploy_retry_count: u32,

    /// Additional options for `cargo build`.
    #[arg(last = true)]
    pub cargo_build_opts: Vec<String>,
}

fn write_profile_config_toml(target_dir: &std::path::Path) -> anyhow::Result<PathBuf> {
    let config_toml_file_path = target_dir.join("profile_config.toml");
    let mut config_toml_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&config_toml_file_path)
        .context("Failed to open profile_config.toml.")?;

    write!(
        config_toml_file,
        "{}",
        toml::to_string_pretty(&*PROFILE_CONFIG_TOML).unwrap()
    )
    .context("Failed to write to the temporary file")?;

    Ok(config_toml_file_path)
}

pub fn start(args: BuildCommand) -> anyhow::Result<()> {
    let Some(metadata) = METADATA.as_ref() else {
        anyhow::bail!("No metadata found. Are you running this command from a workspace?");
    };

    let keyboard_crate_dir = {
        let mut specified_path = std::path::PathBuf::from(&args.path).canonicalize()?;
        loop {
            let cargo_toml_path = specified_path.join("Cargo.toml");
            if cargo_toml_path.exists() {
                break specified_path;
            }

            let Some(p) = specified_path.parent() else {
                anyhow::bail!("Cargo.toml not found. This should be bug.")
            };
            specified_path = p.to_path_buf();
        }
    };

    let keyboard_build_config = {
        if let Ok(str) = std::fs::read_to_string(keyboard_crate_dir.join("rktk.build.json")) {
            let keyboard_build_config: BuildConfig =
                serde_json::from_str(&str).context("Invalid rktk.build.json file")?;
            Some(keyboard_build_config)
        } else {
            None
        }
    };

    let mcu = if let Some(mcu) = args.mcu {
        mcu
    } else if let Some(Some(mcu)) = keyboard_build_config.as_ref().map(|c| c.mcu) {
        mcu
    } else {
        anyhow::bail!(
            "Neither config or command line args doesn't specify mcu. Please specify it."
        );
    };

    let profile_name = if let Some(profile) = args.profile {
        profile
    } else if let Some(Some(profile)) = keyboard_build_config.as_ref().map(|c| c.profile) {
        profile
    } else {
        xprintln!(
            "Neither config or command line args doesn't specify profile. Using `min-size` profile."
        );
        BuildProfileList::MinSize
    };

    let mcu_config = match mcu {
        BuildMcu::Rp2040 => mcu::MCU_CONFIG_RP2040,
        BuildMcu::Nrf52840 => mcu::MCU_CONFIG_NRF52840,
    };
    let profile = profile_name.get_profile();
    let config_toml_file_path = write_profile_config_toml(metadata.target_directory.as_std_path())?;

    let mut cmd_args = vec![
        "--config".to_string(),
        config_toml_file_path.to_string_lossy().to_string(),
        "build".to_string(),
        "--message-format=json-render-diagnostics".to_string(),
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

    cmd_args.extend(args.cargo_build_opts);

    eprintln!();
    xprintln!(
        "Building with profile `{}`, target `{}`\n\tat  : {:?}\n\targs: {:?}",
        profile.name.cyan(),
        format!("{:?}", mcu).cyan(),
        keyboard_crate_dir,
        cmd_args
    );
    eprintln!();

    let cmd = duct::cmd("cargo", cmd_args)
        .stdout_capture()
        .dir(keyboard_crate_dir);

    let output = cmd.run()?;
    let stdout = String::from_utf8(output.stdout)?;

    #[derive(serde::Deserialize)]
    struct CargoBuildCompilerArtifactLog {
        reason: String,
        executable: String,
    }

    let mut artifact_path = None;
    for line in stdout.lines().rev() {
        if let Ok(val) = serde_json::from_str::<CargoBuildCompilerArtifactLog>(line) {
            if val.reason == "compiler-artifact" {
                artifact_path = Some(val.executable);
                break;
            }
        }
    }
    let Some(artifact_path) = artifact_path else {
        anyhow::bail!("Failed to find the artifact path in the cargo output");
    };

    xprintln!(
        "Binary file generated at: {} ({})",
        artifact_path,
        get_bytes(&artifact_path)
    );

    if !args.no_uf2 || args.deploy_dir.is_some() {
        let uf2_path = uf2::elf2uf2(
            &PathBuf::from(artifact_path),
            mcu_config.uf2_family_id,
            mcu_config.uf2_start_addr,
        )?;

        if let Some(deploy_path) = args.deploy_dir {
            let deploy_path = PathBuf::from(deploy_path).join(uf2_path.file_name().unwrap());

            let bar = indicatif::ProgressBar::new(0)
                .with_prefix(format!("{} ", " rktk ".on_blue()))
                .with_style(
                    indicatif::ProgressStyle::with_template(
                        "{prefix} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                    )
                    .unwrap(),
                )
                .with_position(0);

            'abandoned: {
                for i in 0..args.deploy_retry_count {
                    bar.set_message(format!(
                        "Copying... (attempt {}/{})",
                        i + 1,
                        args.deploy_retry_count
                    ));
                    match fs_extra::file::copy_with_progress(
                        &uf2_path,
                        &deploy_path,
                        &fs_extra::file::CopyOptions::new(),
                        |p| {
                            bar.set_length(p.total_bytes);
                            bar.set_position(p.copied_bytes);
                        },
                    ) {
                        Ok(_) => {
                            bar.finish();
                            break 'abandoned;
                        }
                        Err(_e) => {
                            std::thread::sleep(std::time::Duration::from_millis(500));
                        }
                    }
                }
                bar.abandon_with_message("Abandoned");
                anyhow::bail!("Failed to copy the uf2 file to the deploy directory");
            }

            xprintln!("Uf2 file copied to: {}", deploy_path.display());
        }
    }

    Ok(())
}

fn get_bytes(p: impl AsRef<std::path::Path>) -> String {
    human_bytes::human_bytes(std::fs::metadata(p).map(|m| m.len()).unwrap_or(0) as f64)
}
