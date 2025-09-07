//! Deals with the interface of the project.

pub mod cli;
pub mod config;
pub mod error_handler;
pub mod scripts;

pub use cli::commands::Cli;
#[cfg(test)]
pub use config::TEST_CONFIG;
pub use config::{CONFIG_FILE, Config, version_config::VersionConfig};
// TODO remove the requirement for some of this being public
