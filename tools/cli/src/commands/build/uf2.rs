use std::path::PathBuf;

use anyhow::Context as _;

use crate::{commands::build::get_bytes, utils::xprintln};

pub fn elf2uf2(
    elf_path: &std::path::Path,
    family_id: u32,
    target_addr: u32,
) -> anyhow::Result<PathBuf> {
    let artifact_dir = elf_path.parent().context("No parent dir in output file")?;
    let artifact_name = elf_path
        .file_stem()
        .context("No file stem in output file")?
        .to_string_lossy();

    // convert elf to bin
    let bin_path = artifact_dir.join(format!("{}.bin", artifact_name));
    duct::cmd!(
        "arm-none-eabi-objcopy",
        "-O",
        "binary",
        &elf_path,
        &bin_path
    )
    .run()
    .context("Failed to convert to bin. Is arm-none-eabi-objcopy installed?")?;
    xprintln!(
        "Bin file generated at: {} ({})",
        bin_path.display(),
        get_bytes(&bin_path)
    );

    let uf2_path = artifact_dir.join(format!("{}.uf2", artifact_name));
    let uf2_data = uf2::bin_to_uf2(&std::fs::read(bin_path)?, family_id, target_addr)?;
    std::fs::write(&uf2_path, uf2_data).context("Failed to write uf2 file")?;
    xprintln!(
        "Uf2 file generated at: {} ({})",
        uf2_path.display(),
        get_bytes(&uf2_path)
    );

    Ok(uf2_path)
}
