//! Deals with user config.

// Don't want `import config::config::Config`
mod config;
pub mod stage_config;
pub mod version_config;
pub mod wiki_config;

#[cfg(test)]
pub use config::DEFAULT_CONFIG;
pub use config::{Config, CONFIG_FILE};
