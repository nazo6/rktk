use duct::cmd;

use crate::utils::xprintln;

const PUBLISH_ORDER: &[&str] = &[
    "rktk-keymanager",
    "rktk-rrp",
    "rktk",
    "rktk-drivers-common",
    "rktk-drivers-nrf",
    "rktk-drivers-rp",
    "rktk-rrp-client-webhid",
    "cli",
];

pub fn start(execute: bool, continue_on_error: bool) -> anyhow::Result<()> {
    xprintln!("Publishing...");

    let mut results = Vec::new();
    for package in PUBLISH_ORDER {
        eprintln!();
        xprintln!("Publishing crate `{}`", package);

        let now = std::time::Instant::now();

        let mut args = vec!["publish", "-p", package, "--features", "_check"];
        if !execute {
            args.push("-n");
            args.push("--allow-dirty");
        }

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
            msg.push_str(&format!("\n  - {}", package));
        }
        anyhow::bail!(msg);
    }

    Ok(())
}
