use anyhow::Context;
use duct::cmd;

use crate::utils::{xprintln, METADATA};

pub fn start(name: String) -> anyhow::Result<()> {
    let Some(metadata) = METADATA.as_ref() else {
        anyhow::bail!("No metadata found. Are you running this command from a workspace?");
    };

    if &name == "all" {
        let mut crates = Vec::new();
        for package in metadata.workspace_packages() {
            crates.push((
                package.manifest_path.parent().context("no parent dir")?,
                package,
            ));
        }
        xprintln!("Checking all {} crates...", crates.len());

        let mut results = Vec::new();
        for (crate_path, package) in crates {
            eprintln!();
            xprintln!("Checking crate `{}` ({})", package.name, crate_path);

            let now = std::time::Instant::now();

            let res = cmd!("cargo", "clippy").dir(crate_path).run();
            let is_err = res.is_err();

            let elapsed = now.elapsed();

            results.push((crate_path, package, res, elapsed));

            if is_err {
                break;
            }
        }

        let mut failed = Vec::new();
        for (crate_path, package, result, duration) in results {
            match result {
                Ok(_) => {
                    xprintln!(
                        "{}  `{}` ({}) in {}s",
                        " PASS ".on_green(),
                        package.name,
                        crate_path,
                        duration.as_secs()
                    );
                }
                Err(err) => {
                    xprintln!(
                        "{} `{}` ({}) in {}s: {}",
                        " ERROR ".on_red(),
                        package.name,
                        crate_path,
                        duration.as_secs(),
                        err.to_string().red()
                    );
                    failed.push((crate_path, package.name.clone()));
                }
            }
        }

        if !failed.is_empty() {
            let mut msg = "Some crates failed to pass clippy: ".to_string();
            for (crate_path, name) in failed {
                msg.push_str(&format!("\n  - {} ({})", name, crate_path));
            }
            anyhow::bail!(msg);
        }
    } else {
        let package = metadata
            .workspace_packages()
            .into_iter()
            .find(|p| p.name == name)
            .context("no such crate")?;
        let dir = package.manifest_path.parent().context("no parent dir")?;

        xprintln!("Checking crate `{}` ({})", package.name, dir);
        cmd!("cargo", "clippy")
            .dir(dir)
            .run()
            .with_context(|| format!("Failed to run clippy for crate: {}", dir))?;
    }

    Ok(())
}
