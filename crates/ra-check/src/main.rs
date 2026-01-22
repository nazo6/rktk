use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use std::env;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    match run() {
        Ok(success) => {
            if success {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<bool> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let (subcommand, cargo_args) = parse_args(args)?;

    // Get workspace metadata
    let metadata = MetadataCommand::new()
        .exec()
        .context("Failed to get cargo metadata")?;

    // Check if we're in a workspace
    if metadata.workspace_members.is_empty() {
        eprintln!("No workspace members found. Are you in a cargo workspace?");
        return Ok(false);
    }

    println!(
        "Running 'cargo {}' on {} workspace member(s)...\n",
        subcommand,
        metadata.workspace_members.len()
    );

    let mut all_success = true;
    let mut failed_members = Vec::new();

    // Iterate through each workspace member
    for package_id in &metadata.workspace_members {
        // Find the package in the packages list
        let package = metadata
            .packages
            .iter()
            .find(|p| &p.id == package_id)
            .context("Failed to find package")?;

        let package_dir = PathBuf::from(&package.manifest_path)
            .parent()
            .context("Failed to get package directory")?
            .to_path_buf();

        println!("Running {} ({})", package.name, package.version);
        println!("  Location: {}", package_dir.display());

        // Run 'cargo <subcommand>' in the package directory
        let mut cmd = Command::new("cargo");
        cmd.arg(&subcommand);
        cmd.args(&cargo_args);
        cmd.current_dir(&package_dir);

        let status = cmd.status().context(format!(
            "Failed to run cargo {} for {}",
            subcommand, package.name
        ))?;

        if status.success() {
            println!("  ✓ Success\n");
        } else {
            println!("  ✗ Failed\n");
            all_success = false;
            failed_members.push(package.name.clone());
        }
    }

    // Print summary
    println!("─────────────────────────────────");
    if all_success {
        println!("All checks passed!");
    } else {
        println!("Some checks failed:");
        for member in &failed_members {
            println!("  ✗ {}", member);
        }
    }

    Ok(all_success)
}

fn parse_args(args: Vec<String>) -> Result<(String, Vec<String>)> {
    let args: Vec<String> = args.into_iter().skip(1).collect();

    if args.is_empty() {
        eprintln!("Error: subcommand required");
        anyhow::bail!("No subcommand specified");
    }

    let subcommand = args[0].clone();
    let cargo_args = args.into_iter().skip(1).collect();
    Ok((subcommand, cargo_args))
}
