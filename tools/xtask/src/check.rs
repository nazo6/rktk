use anyhow::Context;
use cargo_metadata::{Package, camino::Utf8Path};
use duct::cmd;

use crate::utils::{METADATA, xprintln};

use super::config::CRATES_CONFIG;

fn build_args(crate_name: &str) -> Vec<String> {
    let config = CRATES_CONFIG
        .crates
        .get(crate_name)
        .cloned()
        .unwrap_or_default();

    let mut args = vec!["hack".to_string()];

    if config.check_build {
        args.push("build".to_string());
    } else {
        args.push("clippy".to_string());
    }

    let mut skip = CRATES_CONFIG.check_skip_global.clone().unwrap_or_default();

    if !config.check_no_powerset {
        args.push("--feature-powerset".to_string());

        if let Some(at_least_one_of) = &config.check_at_least_one_of {
            for group in at_least_one_of {
                args.push("--at-least-one-of".to_string());
                args.push(group.join(","));
            }
        }

        if let Some(group_features) = &config.check_group_features {
            for group in group_features {
                args.push("--group-features".to_string());
                args.push(group.join(","));
            }
        }

        if let Some(me_features) = &config.check_mutually_exclusive_features {
            for group in me_features {
                args.push("--mutually-exclusive-features".to_string());
                args.push(group.join(","));
            }
        }

        if let Some(skip_features) = &config.check_skip {
            skip = skip_features.clone();
        }
        if !skip.is_empty() {
            args.push("--skip".to_string());
            args.push(skip.join(","));
        }
    }

    if let Some(features) = &config.check_features {
        args.push("--features".to_string());
        args.push(features.join(","));
    }

    args
}

fn run_check(package: &Package) -> (&Utf8Path, Result<std::process::Output, std::io::Error>) {
    let envs = CRATES_CONFIG.check_env.clone();

    let crate_path = package
        .manifest_path
        .parent()
        .context("no parent dir")
        .unwrap();

    let mut cmd = cmd("cargo", build_args(&package.name));

    eprintln!("command: {cmd:?}, envs: {envs:?}");

    for (key, value) in envs {
        cmd = cmd.env(key, value);
    }
    let cmd = cmd.dir(crate_path);

    let res = cmd.run();

    (crate_path, res)
}

pub fn start(name: String) -> anyhow::Result<()> {
    let Some(metadata) = METADATA.as_ref() else {
        anyhow::bail!("No metadata found. Are you running this command from a workspace?");
    };

    if &name == "all" {
        let packages = metadata.workspace_packages();
        xprintln!("Checking all {} crates...", packages.len());

        let mut results = Vec::new();
        for package in packages {
            eprintln!();
            xprintln!(
                "Checking crate `{}` ({})",
                package.name,
                package.manifest_path
            );

            let now = std::time::Instant::now();

            let (crate_path, res) = run_check(package);

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
                msg.push_str(&format!("\n  - {name} ({crate_path})"));
            }
            anyhow::bail!(msg);
        }
    } else {
        let package = metadata
            .workspace_packages()
            .into_iter()
            .find(|p| p.name.as_str() == name)
            .context("no such crate")?;
        let dir = package.manifest_path.parent().context("no parent dir")?;

        xprintln!("Checking crate `{}` ({})", package.name, dir);

        run_check(package)
            .1
            .with_context(|| format!("Failed to run clippy for crate: {dir}"))?;
    }

    Ok(())
}
