mod config;
mod deploy;
mod mcu;
mod profile;
mod uf2;

use anyhow::Context as _;
use config::BuildConfig;
use mcu::BuildMcuList;
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
    pub mcu: Option<BuildMcuList>,

    /// Profile to use for building the binary. This internally use cargo profile, but they are different things.
    /// If not specified, `min-size` will be used.
    /// Overrides value in `rktk.build.json`
    #[arg(long, short, value_enum, verbatim_doc_comment)]
    pub profile: Option<BuildProfileList>,

    /// Deploy the uf2 binary to the specified path
    #[arg(long, short, group = "deploy", group = "uf2")]
    pub deploy_uf2: Option<String>,

    /// Deploys the binary using probe-rs
    #[arg(long, group = "deploy")]
    pub deploy_probe: bool,

    /// Doesn't generate uf2 file.
    #[arg(long, group = "uf2")]
    pub no_uf2: bool,
    /// Retry count for deploying the binary.
    #[arg(long, default_value_t = 40)]
    pub deploy_retry_count: u32,

    /// Additional options for `cargo build`.
    #[arg(last = true)]
    pub cargo_build_opts: Vec<String>,
}

fn write_profile_config_toml(target_dir: &std::path::Path) -> anyhow::Result<PathBuf> {
    if !target_dir.exists() {
        std::fs::create_dir_all(target_dir).context("Failed to create target directory")?;
    }
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
        anyhow::bail!("No metadata found. Are you running build in rust project?");
    };

    let (package, keyboard_crate_dir) = {
        let mut crr_package = None;
        let keyboard_crate_dir = {
            let mut specified_path = dunce::canonicalize(std::path::PathBuf::from(&args.path))?;
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
        for package in &metadata.packages {
            if dunce::canonicalize(&package.manifest_path).unwrap()
                == keyboard_crate_dir.join("Cargo.toml")
            {
                crr_package = Some(package);
                break;
            }
        }

        if let Some(package) = crr_package {
            (package, keyboard_crate_dir)
        } else {
            anyhow::bail!("Failed to find the package in the metadata");
        }
    };

    let keyboard_build_config = {
        if let Some(val) = package.metadata.get("rktk-cli") {
            let keyboard_build_config: BuildConfig = serde_json::from_value(val.clone())
                .context("Invalid rktk metadata type in Cargo.toml")?;
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

    let mcu_config = mcu.get_mcu_config();
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

    let mut elf_path = None;
    for line in stdout.lines().rev() {
        if let Ok(val) = serde_json::from_str::<CargoBuildCompilerArtifactLog>(line) {
            if val.reason == "compiler-artifact" {
                elf_path = Some(val.executable);
                break;
            }
        }
    }
    let Some(elf_path) = elf_path else {
        anyhow::bail!("Failed to find the artifact path in the cargo output");
    };
    xprintln!(
        "ELF file generated at: {} ({})",
        elf_path,
        get_bytes(&elf_path)
    );

    let elf_path = dunce::canonicalize(PathBuf::from(elf_path))?;

    if !args.no_uf2 || args.deploy_uf2.is_some() {
        let uf2_path = uf2::elf2uf2(
            &elf_path,
            mcu_config.uf2_family_id,
            mcu_config.uf2_start_addr,
        )?;

        if let Some(deploy_path) = args.deploy_uf2 {
            deploy::deploy_uf2(deploy_path, uf2_path, args.deploy_retry_count)?;
        }
    }

    if args.deploy_probe {
        deploy::deploy_probe(&elf_path)?;
    }

    Ok(())
}

fn get_bytes(p: impl AsRef<std::path::Path>) -> String {
    human_bytes::human_bytes(std::fs::metadata(p).map(|m| m.len()).unwrap_or(0) as f64)
}
