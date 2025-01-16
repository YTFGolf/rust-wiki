use super::{cli::CliCommand, encounters_cli::EncountersOptions, stage_cli::StageInfoOptions};
use crate::config2::config2::Config;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand, PartialEq)]
/// Which program to run.
pub enum Command {
    #[command(visible_aliases(["stage"]))]
    /// Get information about a stage.
    StageInfo(StageInfoOptions),

    /// Get a list of stages certain enemies appear in.
    Encounters(EncountersOptions),
    // #[command(visible_aliases(["wiki", "get"]))]
    // /// Get data from the wiki.
    // ReadWiki(UserConfigCli),
}

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
/// Top-level cli arguments.
pub struct Cli {
    #[command(subcommand)]
    /// Command to use.
    pub command: Command,
    // #[command(flatten)]
    // /// User config.
    // potential feature: split this up, i.e. Config has everything, StageInfo
    // has data mines and suppress, ReadWiki has username. Would require more
    // complexity on the actual Config.
}

impl Cli {
    /// Execute the cli.
    pub fn exec(self, config: Config) {
        match self.command {
            Command::StageInfo(options) => options.run(config),
            Command::Encounters(options) => options.run(config),
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
