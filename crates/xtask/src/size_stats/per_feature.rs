use anyhow::Context as _;
use comfy_table::Table;
use duct::cmd;
use human_bytes::human_bytes;

use crate::{
    size_stats::crate_stats::build_and_get_binary_size,
    utils::{METADATA, xprintln},
};

const TARGET_CRATE: &str = "dummy-nrf";

static FEATURES: &[&str] = &[
    "ble-none",
    "alloc,ble-none",
    "debounce,ble-none",
    "display,ble-none",
    "encoder,ble-none",
    "log-defmt,ble-none",
    "log-log,ble-none",
    "mouse,ble-none",
    "rgb,ble-none",
    "rrp,ble-none",
    "split,ble-none",
    "usb,ble-none",
    "ble-trouble",
];

pub fn start() -> anyhow::Result<()> {
    let Some(metadata) = METADATA.as_ref() else {
        anyhow::bail!("No metadata found. Are you running this command from a workspace?");
    };

    let package = metadata
        .workspace_packages()
        .into_iter()
        .find(|p| p.name.as_str() == TARGET_CRATE)
        .context("no such crate")?;
    let dir = package.manifest_path.parent().context("no parent dir")?;

    xprintln!("Analyzing crate `{}` ({})", package.name, dir);
    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
        .set_header(vec!["features", "elf size", "uf2 size", "diff"]);

    let mut baseline = 0;
    for feature in FEATURES {
        let (_elf_path, bin_file_size, uf2_file_size) = build_and_get_binary_size(
            cmd!(
                "cargo",
                "build",
                "--message-format=json",
                "--release",
                "--features",
                feature
            )
            .dir(dir),
        )?;
        let mut row = vec![
            feature.to_string(),
            format!(
                "{} bytes ({})",
                bin_file_size,
                human_bytes(bin_file_size as f64)
            ),
            format!(
                "{} bytes ({})",
                uf2_file_size,
                human_bytes(uf2_file_size as f64)
            ),
        ];
        if baseline == 0 {
            baseline = bin_file_size;
            row.push("-".to_string());
        } else {
            row.push(format!(
                "{} bytes ({})",
                bin_file_size as i64 - baseline as i64,
                human_bytes((bin_file_size as i64 - baseline as i64) as f64)
            ));
        }
        table.add_row(row);
    }

    println!("{}", table);

    Ok(())
}
