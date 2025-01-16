use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Base options.
pub struct BaseOptions {
    #[arg(short)]
    log: Option<String>,
}
