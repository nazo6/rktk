use crate::{SizeStatsArgs, SizeStatsCommands};

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
        SizeStatsCommands::PerFeature => per_feature::start(),
    }
}
