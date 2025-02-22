use std::path::{Path, PathBuf};

use colored::Colorize as _;
use duct::cmd;

use crate::utils::xprintln;

pub fn deploy_probe(elf_path: &PathBuf) -> anyhow::Result<()> {
    let _ = cmd!("probe-rs", "run", elf_path).run()?;
    Ok(())
}

pub fn deploy_uf2(
    deploy_path_args: String,
    uf2_path: PathBuf,
    deploy_retry_count: u32,
) -> anyhow::Result<()> {
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
        for i in 0..deploy_retry_count {
            if i > 0 {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }

            bar.set_message(format!(
                "Deploying (attempt {}/{})",
                i + 1,
                deploy_retry_count
            ));

            let Ok(deploy_path) = get_uf2_deploy_path(deploy_path_args.clone(), &uf2_path) else {
                continue;
            };

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

                    xprintln!("Success");
                    break 'abandoned;
                }
                Err(_e) => {}
            }
        }
        bar.abandon_with_message("Abandoned");
        anyhow::bail!("Failed to copy the uf2 file to the deploy directory");
    }

    Ok(())
}

fn get_uf2_deploy_path(deploy_path: String, uf2_path: &Path) -> anyhow::Result<PathBuf> {
    let deploy_dir = if deploy_path == "auto" {
        // search mount that have "INFO_UF2.txt" file
        let mount_path = PathBuf::from("/mnt");
        if !mount_path.exists() {
            anyhow::bail!("No /mnt directory found");
        }
        let mut deploy_dir = None;
        for entry in std::fs::read_dir(&mount_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.join("INFO_UF2.TXT").exists() {
                deploy_dir = Some(path);
                break;
            }
        }
        if let Some(deploy_dir) = deploy_dir {
            deploy_dir
        } else {
            anyhow::bail!("No mount found that have INFO_UF2.TXT file");
        }
    } else {
        PathBuf::from(deploy_path)
    };
    Ok(deploy_dir.join(uf2_path.file_name().unwrap()))
}
