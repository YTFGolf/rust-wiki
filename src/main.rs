use clap::{Args, Parser, Subcommand};
use rust_wiki::{
    config::{get_config, Config},
    data::stage::parsed::stage::Stage,
    wiki_files::update_wiki_files,
    wikitext::stage_info::get_stage_info,
};
use std::io::{self, Write};

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Args, PartialEq)]
struct StageInfo {
    selector: Vec<String>,
}

#[derive(Debug, Args, PartialEq)]
struct UserConfigCli {
    #[arg(short, long)]
    path: Option<String>,
}

#[derive(Debug, Subcommand, PartialEq)]
enum Command {
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

/*
TODO

- Remove static CONFIG variable and replace with borrow passed everywhere
  - Maybe for testing have a test_config static
- Remove stage new function
- Move this stuff to `cli.rs`
- Allow cmd options to override user config options
*/

// TODO StageInfo is a bad name
fn stage_info(info: StageInfo, config: &Config) {
    println!("{info:?}");
    let selector = match info.selector.len() {
        1 => {
            let mut s = info.selector;
            // So this becomes an explicit mutable move (i.e. makes it so
            // swap_remove actually returns an owned string)
            s.swap_remove(0)
        }
        0 => {
            print!("Input file selector: ");
            io::stdout().flush().unwrap();
            io::stdin().lines().next().unwrap().unwrap()
            // essentially Python's `input("Input file selector: ")`
        }
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

fn update_config(config: Option<Config>, args: UserConfigCli) {}

fn main() {
    let cli = Cli::parse();
    // FIXME currently assumes that if config is None then the file doesn't
    // exist, but could also be an error with toml parsing if toml doesn't
    // contain every field.
    let config = get_config();

    match cli.command {
        Command::ReadWiki => update_wiki_files(&config.unwrap()),
        Command::StageInfo(si) => stage_info(si, &config.unwrap()),
        Command::Config(args) => update_config(config, args),
    }
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use std::sync::LazyLock;

    static CONFIG: LazyLock<Config> = LazyLock::new(|| get_config().unwrap());

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
