use anyhow::Context as _;

pub fn start(saved_file: String) -> anyhow::Result<()> {
    let mut path = std::path::PathBuf::from(saved_file.clone());

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

    duct::cmd!("cargo", "clippy", "--message-format=json")
        .dir(path)
        .run()?;

    Ok(())
}
