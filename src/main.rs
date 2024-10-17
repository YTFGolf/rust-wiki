use clap::{Parser, Subcommand};
use rust_wiki::{
    data::stage::parsed::stage::Stage, wiki_files::update_wiki_files,
    wikitext::stage_info::get_stage_info,
};
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(visible_aliases(["stage"]))]
    /// Get information about a stage.
    StageInfo,

    #[command(visible_aliases(["wiki", "get"]))]
    /// Get data from the wiki.
    ReadWiki,
}

/*
TODO

    if cli.command.is_none() {
        panic!("Command line args not found!")
    }

- Allow command to be entered via input
- Properly sort out the different commands into their own functions
- Allow selector to either be entered via input or via cmd args
- Add user-config.toml and move config to there
- Allow cmd options to override user config options
*/

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::ReadWiki => update_wiki_files(),
        _ => (),
    }

    print!("Input file selector: ");
    io::stdout().flush().unwrap();
    let selector = io::stdin().lines().next().unwrap().unwrap();
    println!("{selector:?}");

    println!("{}", get_stage_info(&Stage::new(&selector).unwrap()))
}
