use duct::cmd;

use crate::utils::xprintln;

const PUBLISH_ORDER: &[(&str, bool)] = &[
    ("rktk-keymanager", false),
    ("rktk-rrp", false),
    ("rktk", false),
    ("rktk-drivers-common", false),
    ("rktk-drivers-nrf", true),
    ("rktk-drivers-rp", false),
    ("rktk-rrp-client-webhid", false),
    ("rktk-cli", false),
];

pub fn start(
    crate_name: Option<String>,
    execute: bool,
    continue_on_error: bool,
) -> anyhow::Result<()> {
    let publish_crates: Vec<(&str, bool)> = if let Some(crate_name) = crate_name {
        PUBLISH_ORDER
            .iter()
            .copied()
            .filter(|s| s.0 == crate_name)
            .collect()
    } else {
        PUBLISH_ORDER.to_vec()
    };

    xprintln!("Publishing...");

    let mut results = Vec::new();
    for (package, disable_default_feature) in publish_crates {
        let now = std::time::Instant::now();

        let mut args = vec!["publish", "-p", &package, "--features", "_check"];
        if disable_default_feature {
            args.push("--no-default-features");
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
