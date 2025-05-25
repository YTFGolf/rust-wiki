//! Contains all CLI commands.

use super::{cli_util::CliCommand, gauntlet_cli::GauntletOptions};
use crate::{
    config::Config,
    interface::scripts::{
        cat_info::cat_cli::CatInfoOptions, encounters::encounters_cli::EncountersOptions,
        map_info::map_cli::MapInfoOptions, read_wiki::wiki_cli::ReadWikiOptions,
        stage_info::stage_cli::StageInfoOptions,
    },
};
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand, PartialEq)]
/// Which program to run.
pub enum Command {
    #[command(visible_aliases(["stage"]))]
    /// Get information about a stage.
    StageInfo(StageInfoOptions),

    /// Get a list of stages certain enemies appear in.
    Encounters(EncountersOptions),

    #[command(visible_aliases(["wiki", "get"]))]
    /// Get data from the wiki.
    ReadWiki(ReadWikiOptions),

    // hidden because it's in beta
    #[command(visible_aliases(["map"]), hide=true)]
    /// Get information about a map.
    MapInfo(MapInfoOptions),

    // hidden because it's in beta
    #[command(visible_aliases(["cat"]), hide=true)]
    /// Get information about a cat.
    CatInfo(CatInfoOptions),

    /// Get most boilerplate for a gauntlet map.
    ///
    /// See <https://battlecats.miraheze.org/wiki/?diff=207709> for a list of
    /// additional stuff you may need to do, although
    /// (gauntlet.py)[<https://battlecats.miraheze.org/wiki/User:TheWWRNerdGuy/scripts#tabber-tabpanel-gauntlet.py-1>]
    /// can fix most problems.
    Gauntlet(GauntletOptions),
}

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
/// Top-level cli arguments.
pub struct Cli {
    #[command(subcommand)]
    /// Command to use.
    pub command: Command,
}

impl Cli {
    /// Execute the cli.
    pub fn exec(self, config: Config) {
        match self.command {
            Command::StageInfo(options) => options.run(config),
            Command::Encounters(options) => options.run(config),
            Command::ReadWiki(options) => options.run(config),
            Command::MapInfo(options) => options.run(config),
            Command::CatInfo(options) => options.run(config),
            Command::Gauntlet(options) => options.run(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_command() {
        const ARGS: [&str; 2] = ["run_program", "invalid-command"];
        let cli = Cli::try_parse_from(ARGS.iter());
        assert!(cli.is_err());
    }
}
