//! Commands for the cli.

use super::user_config::UserConfigCli;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct StageInfoOptions {
    /// Stage selector.
    // TODO put a proper place for docs here.
    pub selector: Vec<String>,

    #[command(flatten)]
    /// User config.
    pub config: UserConfigCli,
}

#[derive(Debug, Args, PartialEq)]
pub struct EncountersOptions {
    pub name: Vec<String>,

    #[command(flatten)]
    /// User config.
    pub config: UserConfigCli,
}

#[derive(Debug, Subcommand, PartialEq)]
/// Which program to run.
pub enum Command {
    #[command(visible_aliases(["stage"]))]
    /// Get information about a stage.
    StageInfo(StageInfoOptions),

    Encounters(EncountersOptions),

    #[command(visible_aliases(["wiki", "get"]))]
    /// Get data from the wiki.
    ReadWiki(UserConfigCli),

    /// Update config.
    Config(UserConfigCli),
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
    // pub config: UserConfigCli,
    // potential feature: split this up, i.e. Config has everything, StageInfo
    // has data mines and suppress, ReadWiki has username. Would require more
    // complexity on the actual Config.
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use crate::{cli::parse::stage_info, config::DEFAULT_CONFIG};

    fn blank_config() -> UserConfigCli {
        UserConfigCli {
            path: None,
            username: None,
            suppress: None,
        }
    }

    #[test]
    fn info_single_full_selector() {
        const ARGS: [&str; 3] = ["run_program", "stage", "l 0 0"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfoOptions {
                    selector: ["l 0 0".to_string()].to_vec(),
                    config: blank_config()
                }),
            }
        );

        let si = match cli.command {
            Command::StageInfo(si) => si,
            _ => unreachable!(),
        };
        stage_info(si, &DEFAULT_CONFIG);
    }

    #[test]
    fn info_multipart_selector() {
        const ARGS: [&str; 5] = ["run_program", "stage", "l", "0", "0"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfoOptions {
                    selector: ["l".to_string(), "0".to_string(), "0".to_string()].to_vec(),
                    config: blank_config()
                }),
            }
        );

        let si = match cli.command {
            Command::StageInfo(si) => si,
            _ => unreachable!(),
        };
        stage_info(si, &DEFAULT_CONFIG);
    }

    #[test]
    fn info_single_selector() {
        const ARGS: [&str; 3] = ["run_program", "stage", "filibuster"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfoOptions {
                    selector: ["filibuster".to_string()].to_vec(),
                    config: blank_config()
                }),
            }
        );

        let si = match cli.command {
            Command::StageInfo(si) => si,
            _ => unreachable!(),
        };
        stage_info(si, &DEFAULT_CONFIG);
    }

    #[test]
    fn invalid_command() {
        const ARGS: [&str; 2] = ["run_program", "invalid-command"];
        let cli = Cli::try_parse_from(ARGS.iter());
        assert!(cli.is_err());
    }
}
