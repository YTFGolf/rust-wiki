//! Commands for the cli.

use super::user_config::UserConfigCli;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct StageInfo {
    /// Stage selector.
    // TODO put a proper place for docs here.
    pub selector: Vec<String>,
}

#[derive(Debug, Subcommand, PartialEq)]
/// Which program to run.
pub enum Command {
    #[command(visible_aliases(["stage"]))]
    /// Get information about a stage.
    StageInfo(StageInfo),

    #[command(visible_aliases(["wiki", "get"]))]
    /// Get data from the wiki.
    ReadWiki,

    /// Update config.
    Config,
    // don't update anything to do with this without updating config.rs
    // ideally this should just take in something from config.rs
    // since config.rs is already terrible practice doesn't matter if it gets
    // mixed with this does it?
}

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
/// Top-level cli arguments.
pub struct Cli {
    #[command(subcommand)]
    /// Command to use.
    pub command: Command,

    #[command(flatten)]
    /// User config.
    pub config: UserConfigCli,
    // UNIMPLEMENTED split this up, i.e. Config has everything, StageInfo has
    // data mines and suppress, ReadWiki has username
    // FIXME have to run `cargo r -- --suppress=true stage a 0 0` for example to
    // get it to work, where it shouldn't matter where you put suppress
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use crate::{cli::parse::stage_info, config::CONFIG};

    #[test]
    fn info_single_full_selector() {
        const ARGS: [&str; 3] = ["run_program", "stage", "l 0 0"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfo {
                    selector: ["l 0 0".to_string()].to_vec()
                }),
                config: UserConfigCli {
                    path: None,
                    username: None,
                    suppress: None
                }
            }
        );

        let si = match cli.command {
            Command::StageInfo(si) => si,
            _ => unreachable!(),
        };
        stage_info(si, &CONFIG);
    }

    #[test]
    fn info_multipart_selector() {
        const ARGS: [&str; 5] = ["run_program", "stage", "l", "0", "0"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfo {
                    selector: ["l".to_string(), "0".to_string(), "0".to_string()].to_vec()
                }),
                config: UserConfigCli {
                    path: None,
                    username: None,
                    suppress: None
                }
            }
        );

        let si = match cli.command {
            Command::StageInfo(si) => si,
            _ => unreachable!(),
        };
        stage_info(si, &CONFIG);
    }

    #[test]
    fn info_single_selector() {
        const ARGS: [&str; 3] = ["run_program", "stage", "filibuster"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfo {
                    selector: ["filibuster".to_string()].to_vec()
                }),
                config: UserConfigCli {
                    path: None,
                    username: None,
                    suppress: None
                }
            }
        );

        let si = match cli.command {
            Command::StageInfo(si) => si,
            _ => unreachable!(),
        };
        stage_info(si, &CONFIG);
    }

    #[test]
    fn invalid_command() {
        const ARGS: [&str; 2] = ["run_program", "invalid-command"];
        let cli = Cli::try_parse_from(ARGS.iter());
        assert!(cli.is_err());
    }
}
