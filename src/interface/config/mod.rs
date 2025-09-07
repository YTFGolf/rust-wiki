//! Deals with user config.

pub mod cat_config;
pub mod config_obj;
pub mod map_config;
pub mod stage_config;
pub mod version_config;
pub mod wiki_config;

#[cfg(test)]
pub use config_obj::TEST_CONFIG;
pub use config_obj::{CONFIG_FILE, Config};
