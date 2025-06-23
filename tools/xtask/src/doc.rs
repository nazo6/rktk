use std::{path::PathBuf, sync::LazyLock};

use anyhow::Context as _;
use cargo_metadata::{
    Package,
    camino::{Utf8Path, Utf8PathBuf},
};
use duct::cmd;
use sha2::{Digest, Sha256};

use crate::{utils::METADATA, xprintln};

use super::config::CRATES_CONFIG;

const BEFORE_CONTENT: &str = include_str!("./doc_before.html");
static HTML_BEFORE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut hasher = Sha256::new();
    hasher.update(BEFORE_CONTENT);
    let hash = hex::encode(hasher.finalize());

    let path = std::env::temp_dir().join(format!("doc_header_{hash}.html"));
    std::fs::write(&path, BEFORE_CONTENT).unwrap();
    path
});
const AFTER_CONTENT: &str = include_str!("./doc_after.html");
static HTML_AFTER_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut hasher = Sha256::new();
    hasher.update(AFTER_CONTENT);
    let hash = hex::encode(hasher.finalize());

    let path = std::env::temp_dir().join(format!("doc_header_{hash}.html"));
    std::fs::write(&path, AFTER_CONTENT).unwrap();
    path
});

fn run_doc(
    p: &Package,
    parts_root: &Utf8Path,
) -> Result<(PathBuf, Utf8PathBuf, PathBuf), anyhow::Error> {
    let part_dir = parts_root.join(p.name.as_str());
    let mut args = vec![
        "doc".to_string(),
        "--no-deps".to_string(),
        "--message-format=json".to_string(),
        "-Zrustdoc-map".to_string(),
    ];
    let crate_dir = p.manifest_path.parent().context("no parent dir")?;
    let features = CRATES_CONFIG
        .doc_features_global
        .clone()
        .unwrap_or_default()
        .join(",");
    if !features.is_empty() {
        args.push("--features".to_string());
        args.push(features);
    }

    let rustdoc_flags = [
        "-Zunstable-options".to_string(),
        "--parts-out-dir".to_string(),
        part_dir.to_string(),
        "--merge=none".to_string(),
        "--html-before-content".to_string(),
        HTML_BEFORE_PATH.to_string_lossy().to_string(),
        "--html-after-content".to_string(),
        HTML_AFTER_PATH.to_string_lossy().to_string(),
    ];

    let output = cmd("cargo", args)
        .stdout_capture()
        .dir(crate_dir)
        .env("RUSTDOCFLAGS", rustdoc_flags.join(" "))
        .run()?;
    let stdout = String::from_utf8(output.stdout)?;

    #[derive(serde::Deserialize)]
    struct CargoDocCompilerArtifactLog {
        reason: String,
        package_id: String,
        filenames: Vec<String>,
    }

    let mut doc_index_html = None;
    for line in stdout.lines().rev() {
        if let Ok(val) = serde_json::from_str::<CargoDocCompilerArtifactLog>(line) {
            if val.reason == "compiler-artifact" && val.package_id == p.id.repr {
                doc_index_html = val
                    .filenames
                    .iter()
                    .find(|f| f.ends_with("index.html"))
                    .cloned();
                break;
            }
        }
    }

    let Some(doc_index_html) = doc_index_html else {
        anyhow::bail!("Failed to find the doc index.html path in the cargo output");
    };
    let doc_index_html = PathBuf::from(doc_index_html);
    let doc_dir = doc_index_html.parent().context("no parent dir")?;

    xprintln!(
        "Doc for `{}` is available at: {}",
        p.name,
        doc_dir.display()
    );

    let src_dir = doc_dir
        .parent()
        .unwrap()
        .join("src")
        .join(p.name.replace("-", "_"));
    dbg!(&src_dir);

    Ok((doc_dir.to_path_buf(), part_dir, src_dir))
}

pub fn start() -> anyhow::Result<()> {
    let Some(metadata) = METADATA.as_ref() else {
        anyhow::bail!("No metadata found. Are you running this command from a workspace?");
    };

    let packages = metadata.workspace_packages();
    xprintln!("Documenting all {} crates...", packages.len());

    let parts_root = metadata.target_directory.join("doc.parts");
    let merged_root = metadata.target_directory.join("doc.merged");

    if merged_root.exists() {
        std::fs::remove_dir_all(&merged_root)?;
    }

    let mut doc_dirs = Vec::new();
    for package in packages {
        let config = CRATES_CONFIG
            .crates
            .get(package.name.as_str())
            .cloned()
            .unwrap_or_default();
        if config.doc_disabled {
            eprintln!();
            xprintln!("Skipping documentation for crate `{}`", package.name);
            continue;
        }

        eprintln!();
        xprintln!(
            "Documenting crate `{}` ({})",
            package.name,
            package.manifest_path
        );

        let (doc_dir, part_dir, src_dir) = run_doc(package, &parts_root)?;
        doc_dirs.push((doc_dir, part_dir, src_dir));
    }

    let mut rustdoc_args = vec![
        "-Z".to_string(),
        "unstable-options".to_string(),
        "--merge=finalize".to_string(),
        "--enable-index-page".to_string(),
        format!("--out-dir={}", merged_root),
        "--extern-html-root-url".to_string(),
        "embassy_sync=https://docs.embassy.dev/embassy-sync/git/default".to_string(),
    ];

    for (doc_dir, part_dir, src_dir) in doc_dirs {
        rustdoc_args.push(format!("--include-parts-dir={part_dir}"));
        let crate_name = doc_dir.file_name().unwrap().to_str().unwrap();
        // copy doc html
        dircpy::CopyBuilder::new::<&str, &str>(
            doc_dir.to_string_lossy().as_ref(),
            merged_root.join(crate_name).as_ref(),
        )
        .run()?;
        // copy src html
        dircpy::CopyBuilder::new::<&str, &str>(
            src_dir.to_string_lossy().as_ref(),
            merged_root.join("src").join(crate_name).as_ref(),
        )
        .run()?;
    }

    cmd("rustdoc", rustdoc_args).run()?;

    let index_html = merged_root.join("index.html");
    std::fs::write(
        &index_html,
        r#"<meta http-equiv="refresh" content="0;URL=rktk/index.html">"#,
    )?;

    std::fs::remove_file(&*HTML_BEFORE_PATH).unwrap();

    Ok(())
}
