use clap::{Args, Parser, Subcommand};
use rust_wiki::{
    data::stage::parsed::stage::Stage, wiki_files::update_wiki_files,
    wikitext::stage_info::get_stage_info,
};
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Args)]
struct StageInfo {
    selector: Vec<String>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(visible_aliases(["stage"]))]
    /// Get information about a stage.
    StageInfo(StageInfo),

    #[command(visible_aliases(["wiki", "get"]))]
    /// Get data from the wiki.
    ReadWiki,
}

/*
TODO

- Add user-config.toml and move config to there
- Allow cmd options to override user config options
*/

fn stage_info(info: StageInfo) {
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
    println!("{}", get_stage_info(&Stage::new(&selector).unwrap()))
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::ReadWiki => update_wiki_files(),
        Command::StageInfo(si) => stage_info(si),
    }
}

/*
Testing clap:
- `"l 0 0"`
- `l 0 0`
- `filibuster`
- `invalid-selector`
*/
