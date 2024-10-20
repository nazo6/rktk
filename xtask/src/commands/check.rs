use anyhow::Context;
use duct::cmd;

use crate::utils::xprintln;

pub fn start(name: String) -> anyhow::Result<()> {
    let metadata = cargo_metadata::MetadataCommand::new().no_deps().exec()?;

    if &name == "all" {
        let mut crate_dirs = Vec::new();
        for package in metadata.workspace_packages() {
            crate_dirs.push(package.manifest_path.parent().context("no parent dir")?);
        }
        xprintln!("Checking all {} crates...", crate_dirs.len());

        let mut results = Vec::new();
        for crate_dir in crate_dirs {
            eprintln!();
            xprintln!("Checking crate at {:?}", crate_dir);

            let now = std::time::Instant::now();

            let res = cmd!("cargo", "clippy",)
                .dir(crate_dir)
                .run()
                .with_context(|| format!("Failed to run clippy for {:?}", crate_dir));

            let elapsed = now.elapsed();

            results.push((crate_dir, res, elapsed));
        }

        let mut failed = Vec::new();
        for (dir, result, duration) in results {
            match result {
                Ok(_) => {
                    xprintln!(
                        "{}  at {} ({}s)",
                        " PASS ".on_blue(),
                        dir,
                        duration.as_secs()
                    );
                }
                Err(err) => {
                    xprintln!(
                        "{} at {} ({}s): {}",
                        " ERROR ".on_red(),
                        dir,
                        duration.as_secs(),
                        err.to_string().red()
                    );
                    failed.push(dir);
                }
            }
        }

        if !failed.is_empty() {
            anyhow::bail!("Some crates failed to pass clippy: {:?}", failed);
        }
    } else {
        let package = metadata
            .workspace_packages()
            .into_iter()
            .find(|p| p.name == name)
            .context("no such crate")?;
        let dir = package.manifest_path.parent().context("no parent dir")?;

        xprintln!("Checking crate at {:?}", dir);
        cmd!("cargo", "clippy",)
            .dir(dir)
            .run()
            .with_context(|| format!("Failed to run clippy for crate: {}", dir))?;
    }

    Ok(())
}
