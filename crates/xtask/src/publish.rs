use duct::cmd;

use crate::{config::CRATES_CONFIG, utils::xprintln};

pub fn start(
    crate_name: Option<String>,
    execute: bool,
    continue_on_error: bool,
) -> anyhow::Result<()> {
    let publish_crates: Vec<&String> = if let Some(crate_name) = crate_name {
        CRATES_CONFIG
            .publish_order
            .iter()
            .filter(|c| **c == crate_name)
            .collect()
    } else {
        CRATES_CONFIG.publish_order.iter().collect()
    };

    xprintln!("Publishing...");

    let mut results = Vec::new();
    for package in publish_crates {
        let now = std::time::Instant::now();
        let use_release_feature = CRATES_CONFIG
            .crates
            .get(package)
            .map(|c| c.use_release_feature)
            .unwrap_or(false);

        let mut args = vec!["publish", "-p", package, "--features"];
        if use_release_feature {
            args.push("_release_check");
        } else {
            args.push("_check");
        }
        if !execute {
            args.push("-n");
            args.push("--allow-dirty");
        }

        xprintln!("Publishing crate `{}`, args: {:?}", package, args);

        let res = cmd("cargo", args).run();
        let is_err = res.is_err();

        let elapsed = now.elapsed();

        results.push((package, res, elapsed));

        if is_err && !continue_on_error {
            break;
        }
    }

    let mut failed = Vec::new();
    for (package, result, duration) in results {
        match result {
            Ok(_) => {
                xprintln!(
                    "{} `{}` in {}s",
                    " PASS ".on_green(),
                    package,
                    duration.as_secs()
                );
            }
            Err(err) => {
                xprintln!(
                    "{} `{}` in {}s: {}",
                    " ERROR ".on_red(),
                    package,
                    duration.as_secs(),
                    err.to_string().red()
                );
                failed.push(package);
            }
        }
    }

    if !failed.is_empty() {
        let mut msg = "Some crates failed to pass clippy: ".to_string();
        for package in failed {
            msg.push_str(&format!("\n  - {package}"));
        }
        anyhow::bail!(msg);
    }

    Ok(())
}
