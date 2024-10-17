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
    selector: Option<String>,
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

- Allow selector to either be entered via input or via cmd args
- Add user-config.toml and move config to there
- Allow cmd options to override user config options
*/

fn stage_info(a: StageInfo) {
    println!("{a:?}");
    print!("Input file selector: ");
    io::stdout().flush().unwrap();
    let selector = io::stdin().lines().next().unwrap().unwrap();
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
