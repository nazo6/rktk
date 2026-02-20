use std::{io::Write, path::PathBuf};

use anyhow::Context;
use duct::cmd;
use human_bytes::human_bytes;

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

    fn new(name: &str, bin: &str, features: &[String], dir: &str) -> Self {
        let mut text = format!("## Stats for `{}`\n", name);
        text.push_str(&format!("- Path: `{}`\n", dir));
        text.push_str(&format!("- Binary: `{}`\n", bin));
        if !features.is_empty() {
            text.push_str(&format!("- Features: `{}`\n", features.join(",")));
        }
        text.push_str("\n\n");
        Self { text }
    }

    fn write(&mut self, title: &str, value: &str, collapse: bool) {
        if collapse {
            self.text
                .push_str(&format!("<details>\n<summary>{title}</summary>\n\n"));
        }
        self.text.push_str(&format!("### {title}\n"));
        self.text.push_str(&format!("```\n{value}\n```\n"));
        if collapse {
            self.text.push_str("</details>\n");
        }
    }

    fn flush(&self) -> anyhow::Result<()> {
        let mut comment_file = std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(Self::STATS_COMMENT_LOC)?;
        if comment_file.metadata()?.len() == 0 {
            writeln!(comment_file, "# Binary stats")?;
        }
        writeln!(comment_file, "{}", self.text)?;

        let len = comment_file.metadata()?.len();
        xprintln!("Comment file length: {}", len);
        if len > 65536 * 4 {
            xprintln!("Warning: Comment file is too large (>256KB). Truncating.");
            comment_file.set_len(65536 * 4)?;
        }

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
    bin: String,
    features: Vec<String>,
    gh_output: bool,
    extra: bool,
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

    let mut ghw = GhStatsWriter::new(&name, &bin, &features, dir.as_ref());

    let mut common_args = vec!["--release".to_string(), "--bin".to_string(), bin];
    if !features.is_empty() {
        common_args.push("--features".to_string());
        common_args.push(features.join(","));
    }

    /* Run cargo build */

    // NOTE: Some analysis are commented out to reduce the comment body size. (Maxmium characters
    // is 65536)

    xprintln!("Building binary...");
    let (elf_path, bin_file_size, uf2_file_size) = build_and_get_binary_size(build_cmd(
        &["cargo", "build", "--message-format=json"],
        &common_args,
        dir,
    ))?;
    let bytes_str = format!(
        "BIN: {} bytes ({})\nUF2: {} bytes ({})",
        bin_file_size,
        human_bytes(bin_file_size as f64),
        uf2_file_size,
        human_bytes(uf2_file_size as f64)
    );
    xprintln!("Binary sizes:\n {}", bytes_str);
    ghw.write("Binary size (bytes)", bytes_str.as_str(), false);

    if extra {
        /* Analyze using `cargo bloat` */

        xprintln!("Analyzing using `cargo bloat`");
        // let cargo_bloat_res = build_cmd(&["cargo", "bloat", "-n", "30"], &common_args, dir).read()?;
        // eprintln!("{}", cargo_bloat_res);
        // ghw.write("Cargo bloat", &cargo_bloat_res, true);

        let cargo_bloat_crate_res = build_cmd(
            &["cargo", "bloat", "--crates", "-n", "20"],
            &common_args,
            dir,
        )
        .read()?;
        eprintln!("{}", cargo_bloat_crate_res);
        ghw.write("Cargo bloat (crates)", &cargo_bloat_crate_res, true);

        /* Analyze using `elf-size-analyze` */

        xprintln!("Analyzing using `elf-size-analyze`");
        let elf_size_analyze_rom_res = cmd(
            "pipx",
            [
                "run",
                "elf-size-analyze",
                "-HaF",
                &elf_path,
                "-w",
                "150",
                "-m",
                "500",
                "--no-color",
            ],
        )
        .read()?;
        eprintln!("{}", elf_size_analyze_rom_res);
        ghw.write(
            "ELF size analyze (ROM, binary usage)",
            &elf_size_analyze_rom_res,
            true,
        );
        let elf_size_analyze_ram_res = cmd(
            "pipx",
            [
                "run",
                "elf-size-analyze",
                "-HaR",
                &elf_path,
                "-w",
                "150",
                "-m",
                "200",
                "--no-color",
            ],
        )
        .read()?;
        eprintln!("{}", elf_size_analyze_ram_res);
        ghw.write(
            "ELF size analyze (RAM, memory usage)",
            &elf_size_analyze_ram_res,
            true,
        );

        /* Analyze using `cargo llvm-lines` */

        xprintln!("Analyzing using `cargo llvm-lines`");
        let llvm_lines_res = build_cmd(&["cargo", "llvm-lines"], &common_args, dir)
            .read()?
            .lines()
            .take(20)
            .collect::<Vec<_>>()
            .join("\n");
        eprintln!("{}", llvm_lines_res);
        ghw.write("LLVM lines", &llvm_lines_res, true);
    }

    if gh_output {
        ghw.flush()?;
    }

    Ok(())
}

pub(super) fn build_and_get_binary_size(
    cmd_ex: duct::Expression,
) -> anyhow::Result<(String, u64, u64)> {
    let cargo_build_res = cmd_ex.read()?;
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

    /* Analyze bin and uf2 files */

    // NOTE: This command is just for stats, so any family name is fine.
    cmd("uf2deploy", ["deploy", "-f", "nrf52840", &elf_path])
        .stderr_capture()
        .read()?;
    let bin_file_size = std::fs::metadata(elf_path.clone() + ".bin")
        .context("failed to get binary file metadata")?
        .len();
    let uf2_file_size = std::fs::metadata(elf_path.clone() + ".uf2")
        .context("failed to get uf2 file metadata")?
        .len();

    Ok((elf_path, bin_file_size, uf2_file_size))
}
