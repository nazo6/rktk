use anyhow::Context as _;
use regex::Regex;

#[derive(Debug, serde::Deserialize)]
pub struct CommentUser {
    id: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Comment {
    commit_id: String,
    body: String,
    user: CommentUser,
}

#[derive(Debug)]
pub struct ParsedComment {
    commit_id: String,
    stats: Vec<ParsedStat>,
}

#[derive(Debug)]
pub struct ParsedStat {
    name: String,
    features: Vec<String>,
    binary_name: String,
    bin_size: usize,
    #[allow(dead_code)]
    uf2_size: usize,
}

impl ParsedStat {
    fn get_label(&self) -> String {
        let mut label = self.name.clone();
        if !self.features.is_empty() {
            label.push_str("_[");
            label.push_str(&self.features.join(","));
            label.push(']');
        }
        label.push_str(&format!("_{}", self.binary_name));

        label
    }
}

pub fn start() -> anyhow::Result<()> {
    let output = duct::cmd!("gh", "api", "repos/nazo6/rktk/comments", "--paginate")
        .read()
        .context("Failed to run gh cli to get comment list")?;
    let comments: Vec<Comment> =
        serde_json::from_str(&output).context("Failed to parse comments JSON")?;
    let parsed_comments: Vec<ParsedComment> = comments
        .into_iter()
        .filter_map(|comment| {
            if comment.user.id != 41898282 {
                eprintln!("Ignoring comment from user id {}", comment.user.id);
                return None;
            }

            let stats = parse_binary_stats(&comment.body);
            if stats.is_empty() {
                eprintln!("No stats found in comment for commit {}", comment.commit_id);
                None
            } else {
                Some(ParsedComment {
                    commit_id: comment.commit_id,
                    stats,
                })
            }
        })
        .collect();

    generate_html(&parsed_comments)?;

    Ok(())
}

fn generate_html(parsed_comments: &[ParsedComment]) -> anyhow::Result<()> {
    let mut config_names = std::collections::HashSet::new();
    for comment in parsed_comments {
        for stat in &comment.stats {
            config_names.insert(stat.get_label());
        }
    }
    let mut config_names: Vec<String> = config_names.into_iter().collect();
    config_names.sort();

    let labels: Vec<String> = parsed_comments
        .iter()
        .map(|c| c.commit_id.split_at(7).0.to_string())
        .collect();

    let mut datasets = Vec::new();
    for name in config_names {
        let mut data = Vec::new();
        for comment in parsed_comments {
            let size = comment
                .stats
                .iter()
                .find(|s| s.get_label() == name)
                .map(|s| s.bin_size);
            data.push(size);
        }

        datasets.push(serde_json::json!({
            "label": name,
            "data": data,
            "tension": 0.1,
            "spanGaps": true
        }));
    }

    let chart_data = serde_json::json!({
        "labels": labels,
        "datasets": datasets
    });

    let html = include_str!("../../res/bin_size_stats.html")
        .replace("CHART_DATA", &chart_data.to_string());

    std::fs::write("target/stats.html", html).context("Failed to write stats.html")?;
    println!("Generated target/stats.html");
    Ok(())
}

pub fn parse_binary_stats(input: &str) -> Vec<ParsedStat> {
    let mut stats = Vec::new();

    let sections = input.split("## Stats for").skip(1);

    let re_name = Regex::new(r"^ `(.+?)`").unwrap();
    let re_binary = Regex::new(r"- Binary: `(.+?)`").unwrap();
    let re_features = Regex::new(r"- Features: `(.+?)`").unwrap();
    let re_bin_size = Regex::new(r"BIN: (\d+) bytes").unwrap();
    let re_uf2_size = Regex::new(r"UF2: (\d+) bytes").unwrap();

    for section in sections {
        let name = re_name
            .captures(section)
            .map(|c| c[1].to_string())
            .unwrap_or_default();

        let binary_name = re_binary
            .captures(section)
            .map(|c| c[1].to_string())
            .unwrap_or_default();

        let mut features: Vec<String> = re_features
            .captures(section)
            .map(|c| c[1].split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
        features.sort();

        let bin_size = re_bin_size
            .captures(section)
            .and_then(|c| c[1].parse().ok())
            .unwrap_or(0);

        let uf2_size = re_uf2_size
            .captures(section)
            .and_then(|c| c[1].parse().ok())
            .unwrap_or(0);

        stats.push(ParsedStat {
            name,
            features,
            binary_name,
            bin_size,
            uf2_size,
        });
    }

    stats
}
