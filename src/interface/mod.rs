//! Deals with the interface of the project.

mod cli;
mod config;
mod error_handler;
mod scripts;

pub use cli::commands::Cli;
pub use config::CONFIG_FILE;
pub use config::Config;
#[cfg(test)]
pub use config::TEST_CONFIG;

pub use config::version_config::Lang as SLang;
// TODO remove the requirement for this to be public
