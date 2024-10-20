mod mcu;
mod profile;

use anyhow::Context as _;
use colored::Colorize as _;
use profile::PROFILE_CONFIG_TOML;
use std::{io::Write as _, path::PathBuf};

use crate::{
    cli::build::{BuildCommand, BuildMcu, BuildProfile},
    utils::{xprintln, METADATA},
};

pub fn start(args: BuildCommand) -> anyhow::Result<()> {
    let Some(metadata) = METADATA.as_ref() else {
        anyhow::bail!("No metadata found. Are you running this command from a workspace?");
    };

    let kb_crate_path = std::path::PathBuf::from(&args.path).canonicalize()?;

    let config_toml_file_path = metadata
        .target_directory
        .clone()
        .join("profile_config.toml");
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
        config_toml_file_path.to_string(),
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
        "Building with profile `{}`, target `{}`\n\targs: {:?}",
        profile.name.cyan(),
        format!("{:?}", args.mcu).cyan(),
        cmd_args
    );
    eprintln!();

    let cmd = duct::cmd("cargo", cmd_args)
        .stdout_capture()
        .dir(kb_crate_path);

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

    if args.uf2 || args.deploy_dir.is_some() {
        let artifact_path = PathBuf::from(artifact_path);
        let artifact_dir = artifact_path
            .parent()
            .context("No parent dir in output file")?;
        let artifact_stem = artifact_path
            .file_stem()
            .context("No file stem in output file")?
            .to_string_lossy();
        let uf2_path = artifact_dir.join(format!("{}.uf2", artifact_stem));

        match args.mcu {
            BuildMcu::Rp2040 => {
                duct::cmd!("elf2uf2-rs", &artifact_path, &uf2_path)
                    .run()
                    .context("Failed to convert to uf2. Is elf2uf2-rs installed?")?;
            }
            BuildMcu::Nrf52840 => {
                let hex_path = artifact_dir.join(format!("{}.hex", artifact_stem));
                duct::cmd!("arm-none-eabi-objcopy", "-Oihex", &artifact_path, &hex_path)
                    .run()
                    .context("Failed to convert to hex. Is arm-none-eabi-objcopy installed?")?;
                xprintln!("Hex file generated at: {}", hex_path.display());
                duct::cmd!(
                    "python3",
                    "-",
                    &hex_path,
                    "-o",
                    &uf2_path,
                    "-c",
                    "-b",
                    "0x26000",
                    "-f",
                    "0xADA52840"
                )
                .stdin_bytes(include_bytes!("build/uf2conv.py"))
                .run()
                .context("Failed to convert to uf2. Is python3 installed?")?;
            }
        }

        xprintln!(
            "Uf2 file generated at: {} ({})",
            uf2_path.display(),
            get_bytes(&uf2_path)
        );

        if let Some(deploy_path) = args.deploy_dir {
            let deploy_path = PathBuf::from(deploy_path).join(uf2_path.file_name().unwrap());

            let bar = indicatif::ProgressBar::new(0)
                .with_prefix(format!("{} ", " xtask ".on_blue()))
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
