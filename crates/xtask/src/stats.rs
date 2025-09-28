use std::{io::Write, path::PathBuf};

use anyhow::Context;
use duct::cmd;

use crate::utils::{METADATA, xprintln};

#[derive(serde::Deserialize)]
pub struct CompilerArtifactLog {
    reason: String,
    executable: String,
}

struct GhStatsWriter {
    text: String,
}

impl GhStatsWriter {
    const STATS_COMMENT_LOC: &str = "/tmp/stats-comment.md";

    fn new(name: &str, bin: &Option<String>, features: &Option<Vec<String>>) -> Self {
        let mut text = format!("## Stats for `{}`", name);
        if let Some(bin) = bin {
            text.push_str(&format!(" (binary `{}`)", bin));
        }
        if let Some(features) = features
            && !features.is_empty()
        {
            text.push_str(&format!(" with features `{}`", features.join(",")));
        }
        text.push_str("\n\n");
        Self { text }
    }

    fn write(&mut self, title: &str, value: &str) {
        self.text.push_str(&format!("### {}\n", title));
        self.text.push_str(&format!("```\n{}\n```\n", value));
    }

    fn flush(&self) -> anyhow::Result<()> {
        let mut comment_file = std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(Self::STATS_COMMENT_LOC)?;
        if comment_file.metadata()?.len() == 0 {
            writeln!(comment_file, "# Binary stats\n")?;
        }
        writeln!(comment_file, "{}", self.text)?;

        Ok(())
    }
}

pub fn build_cmd(
    cmd_array: &[&str],
    args: &[String],
    dir: impl Into<PathBuf> + std::fmt::Debug,
) -> duct::Expression {
    let cmd = cmd(
        cmd_array[0],
        cmd_array[1..]
            .iter()
            .copied()
            .chain(args.iter().map(|a| a.as_str())),
    );
    xprintln!("Running command: {:?} at {:?}", cmd, dir);
    cmd.dir(dir)
}

pub fn start(
    name: String,
    bin: Option<String>,
    features: Option<Vec<String>>,
    gh_output: bool,
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

    let mut ghw = GhStatsWriter::new(&name, &bin, &features);

    let mut common_args = vec!["--release".to_string()];
    if let Some(bin_name) = bin {
        common_args.push("--bin".to_string());
        common_args.push(bin_name.to_string());
    }
    if let Some(features) = &features
        && !features.is_empty()
    {
        common_args.push("--features".to_string());
        common_args.push(features.join(","));
    }

    /* Run cargo build */

    xprintln!("Building binary...");
    let cargo_build_res = build_cmd(
        &["cargo", "build", "--message-format=json"],
        &common_args,
        dir,
    )
    .read()?;
    let mut elf_path = None;
    for line in cargo_build_res.lines() {
        if let Ok(log) = serde_json::from_str::<CompilerArtifactLog>(line)
            && log.reason == "compiler-artifact"
        {
            elf_path = Some(log.executable);
            break;
        }
    }
    let Some(elf_path) = elf_path else {
        anyhow::bail!("Failed to find the binary path in the cargo output");
    };
    xprintln!("Binary path: {}", elf_path);

    /* Analyze bin and uf2 files */

    cmd("uf2deploy", ["deploy", "-f", "nrf52840", &elf_path])
        .stderr_capture()
        .read()?;
    let bin_file_size = std::fs::metadata(elf_path.clone() + ".bin")
        .context("failed to get binary file metadata")?
        .len();
    let uf2_file_size = std::fs::metadata(elf_path.clone() + ".uf2")
        .context("failed to get uf2 file metadata")?
        .len();
    xprintln!(
        "Binary file sizes: BIN={} bytes, UF2={} bytes",
        bin_file_size,
        uf2_file_size
    );
    ghw.write(
        "File size (bytes)",
        format!("BIN: {}\nUF2: {}", bin_file_size, uf2_file_size).as_str(),
    );

    /* Analyze using cargo llvm-lines */

    xprintln!("Analyzing using `cargo llvm-lines`");
    let llvm_lines_res = build_cmd(&["cargo", "llvm-lines"], &common_args, dir)
        .read()?
        .lines()
        .take(20)
        .collect::<Vec<_>>()
        .join("\n");
    xprintln!("{}", llvm_lines_res);
    ghw.write("LLVM lines", &llvm_lines_res);

    if gh_output {
        ghw.flush()?;
    }

    Ok(())
}
