use anyhow::Context as _;
use comfy_table::Table;
use duct::cmd;
use human_bytes::human_bytes;

use crate::{
    size_stats::{GhStatsWriter, crate_stats::build_and_get_binary_size},
    utils::{METADATA, xprintln},
};

const TARGET_CRATE: &str = "dummy-nrf";

static FEATURES: &[&[&str]] = &[
    &["ble-none"],
    &["_check"],
    &["alloc", "ble-none"],
    &["debounce", "ble-none"],
    &["display", "ble-none"],
    &["encoder", "ble-none"],
    &["log-defmt", "ble-none"],
    &["log-log", "ble-none"],
    &["mouse", "ble-none"],
    &["rgb", "ble-none"],
    &["rrp", "ble-none"],
    &["split", "ble-none"],
    &["usb", "ble-none"],
    &["ble-trouble"],
];

const IGNORED_FEATURES: &[&str] = &["ble-none"];

pub fn start(gh_output: bool) -> anyhow::Result<()> {
    let Some(metadata) = METADATA.as_ref() else {
        anyhow::bail!("No metadata found. Are you running this command from a workspace?");
    };

    let package = metadata
        .workspace_packages()
        .into_iter()
        .find(|p| p.name.as_str() == TARGET_CRATE)
        .context("no such crate")?;
    let dir = package.manifest_path.parent().context("no parent dir")?;

    let mut ghw = GhStatsWriter::new_raw("Per-feature size stats");

    xprintln!("Analyzing crate `{}` ({})", package.name, dir);
    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
        .set_header(vec!["features", "elf size", "uf2 size", "diff"]);

    let mut baseline = 0;
    for feature in FEATURES {
        let (_elf_path, bin_size, uf2_size) = build_and_get_binary_size(
            cmd!(
                "cargo",
                "build",
                "--message-format=json",
                "--release",
                "--features",
                feature.join(",")
            )
            .dir(dir),
        )?;
        let mut row = vec![
            feature
                .iter()
                .filter(|f| !IGNORED_FEATURES.contains(f))
                .copied()
                .collect::<Vec<_>>()
                .join(","),
            format!("{} bytes ({})", bin_size, human_bytes(bin_size as f64)),
            format!("{} bytes ({})", uf2_size, human_bytes(uf2_size as f64)),
        ];
        if baseline == 0 {
            baseline = bin_size;
            row.push("-".to_string());
        } else {
            row.push(format!(
                "{} bytes ({})",
                bin_size as i64 - baseline as i64,
                human_bytes((bin_size as i64 - baseline as i64) as f64)
            ));
        }
        table.add_row(row);
    }

    println!("{}", table);
    ghw.write(
        &format!("Crate `{}`", TARGET_CRATE),
        &table.to_string(),
        false,
    );
    if gh_output {
        ghw.flush()?;
    }

    Ok(())
}
