use std::io::Write as _;

use crate::{SizeStatsArgs, SizeStatsCommands, utils::xprintln};

mod crate_stats;
mod gen_history;
mod per_feature;

pub fn start(args: SizeStatsArgs) -> anyhow::Result<()> {
    match args.command {
        SizeStatsCommands::Crate {
            crate_name,
            bin,
            features,
            gh_output,
            extra,
        } => crate_stats::start(crate_name, bin, features, gh_output, extra),
        SizeStatsCommands::GenHistory => gen_history::start(),
        SizeStatsCommands::PerFeature { gh_output } => per_feature::start(gh_output),
    }
}

struct GhStatsWriter {
    text: String,
}

impl GhStatsWriter {
    const STATS_COMMENT_LOC: &str = "/tmp/stats-comment.md";

    fn new(name: &str, bin: &str, features: &[String], dir: &str) -> Self {
        let mut text = format!("## Stats for `{}`\n", name);
        text.push_str(&format!("- Path: `{}`\n", dir));
        text.push_str(&format!("- Binary: `{}`\n", bin));
        if !features.is_empty() {
            text.push_str(&format!("- Features: `{}`\n", features.join(",")));
        }
        text.push_str("\n\n");
        Self { text }
    }

    fn new_raw(name: &str) -> Self {
        let mut text = format!("## {}\n", name);
        text.push_str("\n\n");
        Self { text }
    }

    fn write(&mut self, title: &str, value: &str, collapse: bool) {
        if collapse {
            self.text
                .push_str(&format!("<details>\n<summary>{title}</summary>\n\n"));
        }
        self.text.push_str(&format!("### {title}\n"));
        self.text.push_str(&format!("```\n{value}\n```\n"));
        if collapse {
            self.text.push_str("</details>\n");
        }
    }

    fn flush(&self) -> anyhow::Result<()> {
        let mut comment_file = std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(Self::STATS_COMMENT_LOC)?;
        if comment_file.metadata()?.len() == 0 {
            writeln!(comment_file, "# Binary stats")?;
        }
        writeln!(comment_file, "{}", self.text)?;

        let len = comment_file.metadata()?.len();
        xprintln!("Comment file length: {}", len);
        if len > 65536 * 4 {
            xprintln!("Warning: Comment file is too large (>256KB). Truncating.");
            comment_file.set_len(65536 * 4)?;
        }

        Ok(())
    }
}
