use anyhow::Context;
use duct::cmd;

use crate::utils::{xprintln, METADATA};

use super::config::CRATES_CONFIG;

fn build_args(crate_name: &str) -> Option<Vec<String>> {
    if let Some(c) = CRATES_CONFIG.crates.get(crate_name) {
        if c.test_enabled {
            let mut args = vec!["test".to_string()];

            let features = CRATES_CONFIG
                .test_features_global
                .clone()
                .unwrap_or_default()
                .join(",");
            if !features.is_empty() {
                args.push("--features".to_string());
                args.push(features);
            }

            return Some(args);
        }
    }

    None
}

pub fn start(name: String) -> anyhow::Result<()> {
    let Some(metadata) = METADATA.as_ref() else {
        anyhow::bail!("No metadata found. Are you running this command from a workspace?");
    };

    if &name == "all" {
        xprintln!("Testing crates...");

        let mut crates = Vec::new();
        for package in metadata.workspace_packages() {
            crates.push((
                package.manifest_path.parent().context("no parent dir")?,
                package,
            ));
        }

        let mut results = Vec::new();
        for (crate_path, package) in crates {
            if let Some(args) = build_args(&package.name) {
                eprintln!();
                xprintln!("Testing crate `{}` ({})", package.name, crate_path);

                let now = std::time::Instant::now();

                let res = cmd("cargo", args).dir(crate_path).run();

                let elapsed = now.elapsed();

                results.push((crate_path, package, res, elapsed));
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
            let mut msg = "Some crates failed to pass test: ".to_string();
            for (crate_path, name) in failed {
                msg.push_str(&format!("\n  - {} ({})", name, crate_path));
            }
            anyhow::bail!(msg);
        }
    } else {
        panic!("Not supported");
        // let package = metadata
        //     .workspace_packages()
        //     .into_iter()
        //     .find(|p| p.name == name)
        //     .context("no such crate")?;
        // let dir = package.manifest_path.parent().context("no parent dir")?;
        //
        // if !TEST_WHITELIST.contains(&package.name.as_str()) {
        //     anyhow::bail!("Crate `{}` is not whitelisted for testing", package.name);
        // }
        //
        // xprintln!("Testing crate `{}` ({})", package.name, dir);
        // cmd("cargo", TEST_ARGS)
        //     .dir(dir)
        //     .run()
        //     .with_context(|| format!("Failed to run test for crate: {}", dir))?;
    }

    Ok(())
}
