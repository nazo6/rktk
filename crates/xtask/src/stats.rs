use std::path::PathBuf;

use anyhow::Context;
use duct::cmd;

use crate::utils::{METADATA, xprintln};

use super::config::CRATES_CONFIG;

#[derive(serde::Deserialize)]
pub struct CompilerArtifactLog {
    reason: String,
    executable: String,
}

pub fn run_cargo_subcommand(
    subcommand: &str,
    bin: &Option<String>,
    features: &Option<Vec<String>>,
    dir: impl Into<PathBuf> + std::fmt::Debug,
    json_output: bool,
) -> anyhow::Result<String> {
    let mut args = vec![subcommand.to_string(), "--release".to_string()];
    if json_output {
        args.push("--message-format=json".to_string());
    }
    if let Some(bin_name) = bin {
        args.push("--bin".to_string());
        args.push(bin_name.to_string());
    }
    if let Some(features) = &features
        && !features.is_empty()
    {
        args.push("--features".to_string());
        args.push(features.join(","));
    }
    let cmd = cmd("cargo", args);

    xprintln!("Executing command: {cmd:?} at {dir:?}");

    let cmd = cmd.dir(dir);

    Ok(cmd.read()?)
}

pub fn start(
    name: String,
    bin: Option<String>,
    features: Option<Vec<String>>,
) -> anyhow::Result<()> {
    let Some(metadata) = METADATA.as_ref() else {
        anyhow::bail!("No metadata found. Are you running this command from a workspace?");
    };

    let package = metadata
        .workspace_packages()
        .into_iter()
        .find(|p| p.name.as_str() == name)
        .context("no such crate")?;
    let dir = package.manifest_path.parent().context("no parent dir")?;

    xprintln!("Analyzing crate `{}` ({})", package.name, dir);

    xprintln!("Analyzing using `cargo build`");
    let cargo_build_res = run_cargo_subcommand("build", &bin, &features, dir, true)?;
    let mut bin_path = None;
    for line in cargo_build_res.lines() {
        if let Ok(log) = serde_json::from_str::<CompilerArtifactLog>(line)
            && log.reason == "compiler-artifact"
        {
            bin_path = Some(log.executable);
            break;
        }
    }
    let Some(bin_path) = bin_path else {
        anyhow::bail!("Failed to find the binary path in the cargo output");
    };
    xprintln!("Binary path: {}", bin_path);
    cmd("uf2deploy", ["deploy", "-f", "nrf52840", &bin_path]).read()?;

    xprintln!("Analyzing using `cargo llvm-lines`");
    let llvm_lines_res = run_cargo_subcommand("llvm-lines", &bin, &features, dir, false)?;
    xprintln!(
        "{}",
        llvm_lines_res
            .lines()
            .take(20)
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(())
}
