use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
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
