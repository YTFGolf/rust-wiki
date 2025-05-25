//! Deals with the interface of the project.

mod cli;
mod config;
mod error_handler;
mod scripts;

pub use cli::commands::Cli;
#[cfg(test)]
pub use config::TEST_CONFIG;
pub use config::{CONFIG_FILE, Config, version_config::Lang as SLang};
// TODO remove the requirement for `SLang` to be public
