use clap::{Args, Parser, Subcommand};

#[derive(Args, Debug, PartialEq)]
pub struct Test1 {
    opt1: String,
    opt2: String,
}
#[derive(Args, Debug, PartialEq)]
pub struct Test2 {
    opt3: String,
    opt4: String,
}
#[derive(Debug, Args, PartialEq)]
pub struct TestHolder {
    #[command(flatten)]
    input: Test1,
    #[command(flatten)]
    options: Test2,
}

#[derive(Debug, Subcommand, PartialEq)]
/// Which program to run.
pub enum Command {
    Test(TestHolder),
    //     // #[command(visible_aliases(["stage"]))]
    //     /// Get information about a stage.
    //     // StageInfo(StageInfoOptions),

    //     /// Get a list of stages certain enemies appear in.
    //     // Encounters(EncountersOptions),

    //     // #[command(visible_aliases(["wiki", "get"]))]
    //     /// Get data from the wiki.
    //     // ReadWiki(UserConfigCli),
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
