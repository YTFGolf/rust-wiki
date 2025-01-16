//! Deals with user config.

mod config_obj;
pub mod stage_config;
pub mod version_config;
pub mod wiki_config;

#[cfg(test)]
pub use config_obj::TEST_CONFIG;
pub use config_obj::{Config, CONFIG_FILE};
