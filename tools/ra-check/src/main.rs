use anyhow::Context as _;
use std::io::Write as _;

fn main() -> anyhow::Result<()> {
    main_inner()?;
    Ok(())
}

fn main_inner() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    let mut log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("target/log.txt")
        .expect("Could not open log file");
    writeln!(log_file, "{:?}", args).expect("Could not write to log file");

    let saved_file = args.get(1).context("No saved file path provided")?;

    let mut path = std::path::PathBuf::from(saved_file);

    loop {
        path = path
            .parent()
            .with_context(|| format!("Could not find Cargo.toml for file: {:?}", saved_file))?
            .to_owned();

        if !path.join("Cargo.toml").exists() {
            continue;
        } else {
            break;
        }
    }

    duct::cmd!(
        "cargo",
        "clippy",
        "--message-format=json",
        "--all-targets",
        "--features",
        "_check"
    )
    .dir(path)
    .run()?;

    Ok(())
}
