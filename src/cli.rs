//! Parses cli arguments for use in main.
use crate::{
    config::{set_config_file, Config, UserConfig, UserVersion},
    data::{stage::parsed::stage::Stage, version::Version},
    wikitext::stage_info::get_stage_info,
};
use clap::{Args, Parser, Subcommand};
use std::{
    fs::File,
    io::{self, Write},
};

fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin().lines().next().unwrap().unwrap()
}

// TODO StageInfo is a bad name
/// Get stage info.
pub fn stage_info(info: StageInfo, config: &Config) {
    println!("{info:?}");
    let selector = match info.selector.len() {
        1 => {
            let mut s = info.selector;
            // So this becomes an explicit mutable move (i.e. makes it so
            // swap_remove actually returns an owned string)
            s.swap_remove(0)
        }
        0 => input("Input file selector: "),
        _ => info.selector.join(" "),
    };
    println!("{selector:?}");
    println!(
        "{}",
        get_stage_info(
            &Stage::new_versioned(&selector, &config.current_version).unwrap(),
            config
        )
    )
}

/// Create user config file. If a config arg is not provided then it is provided
/// by input.
fn create_config(args: UserConfigCli) {
    let path = match args.path {
        Some(p) => p,
        None => input("Enter root directory of decrypted files: "),
    };

    let lang = match Version::get_lang(&path) {
        None => Some(input("Enter language: ")),
        Some(language) => {
            let prompt = format!("Enter language (default: {language}): ");
            let l = input(&prompt);
            if l.is_empty() {
                None
            } else {
                Some(l)
            }
        }
    };

    let version_number = match Version::get_version_number(&path) {
        None => Some(input("Enter version number: ")),
        Some(number) => {
            let prompt = format!("Enter version number (default: {number}): ");
            let n = input(&prompt);
            if n.is_empty() {
                None
            } else {
                Some(n)
            }
        }
    };

    let version = UserVersion {
        path,
        lang,
        number: version_number,
    };

    let name = match args.username {
        Some(name) => name,
        None => input("Enter wiki username: "),
    };

    let suppress = match args.suppress {
        Some(suppress) => suppress,
        None => false,
    };

    let user_config = UserConfig {
        version,
        username: name,
        suppress_gauntlet_magnification: suppress,
    };

    let toml_repr = toml::to_string(&user_config).unwrap();
    set_config_file(&toml_repr);
}

/// Update user config.
pub fn update_config(config: Option<UserConfig>, args: UserConfigCli) {
    let mut config = match config {
        None => return create_config(args),
        Some(c) => c,
    };

    // merge configs
    if let Some(path) = args.path {
        config.version.path = path;
    }
    if let Some(name) = args.username {
        config.username = name;
    }
    if let Some(s) = args.suppress {
        config.suppress_gauntlet_magnification = s;
    }

    let toml_repr = toml::to_string(&config).unwrap();
    set_config_file(&toml_repr);
}

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct StageInfo {
    selector: Vec<String>,
}

#[derive(Debug, Args, PartialEq)]
/// User config options.
pub struct UserConfigCli {
    #[arg(short, long)]
    path: Option<String>,

    #[arg(short = 'n', long)]
    username: Option<String>,

    #[arg(long)]
    suppress: Option<bool>,
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
    Config(UserConfigCli),
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
}

/*
TODO

- ~~Remove static CONFIG variable and replace with borrow passed everywhere~~
  - Maybe for testing have a test_config static
  - Rename old CONFIG and Stage::new
- Implement create and update config
- Allow cmd options to override user config options
*/

#[cfg(test)]
mod cli_tests {
    use super::*;
    use crate::config::CONFIG;

    #[test]
    fn info_single_full_selector() {
        const ARGS: [&str; 3] = ["run_program", "stage", "l 0 0"];
        let cli = Cli::parse_from(ARGS.iter());
        assert_eq!(
            cli,
            Cli {
                command: Command::StageInfo(StageInfo {
                    selector: ["l 0 0".to_string()].to_vec()
                })
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
                })
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
                })
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
