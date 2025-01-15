//! Represents the config.

use super::{stage_config::StageConfig, version_config::VersionConfig, wiki_config::WikiConfig};
use log::Level;
use serde::{Deserialize, Serialize};

/*
# Outline

## Config
- Actual config
- Object can do its own serialisation and deserialisation
- Ideally also includes the comments inside the file
- Initialising config comes with default values
- If values are missing then tells you which ones are missing and gives option
  to fill in defaults (could just generate default then merge with current
  config, or maybe tells you how to fill them out, or somehow figures out how to
  fill them out).
- main/cmd will deal with combining user config and cmd config (maybe a trait
  each subcommand option type could implement)
*/

/// Necessary to make [`Level`] serialise as lower case.
fn serialize_log_level<S>(level: &Level, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&level.as_str().to_lowercase())
}

#[derive(Debug, Serialize, Deserialize)]
/// Configuration
// TODO remove Serialise and replace with toml-edit.
pub struct Config {
    /// Level of log warning.
    #[serde(serialize_with = "serialize_log_level")]
    log_level: Level,
    /// Wiki config.
    wiki: WikiConfig,
    /// Game version config.
    version: VersionConfig,
    /// Config for `stage_info`.
    stage_info: StageConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: Level::Info,
            // grr
            wiki: Default::default(),
            version: Default::default(),
            stage_info: Default::default(),
        }
    }
}
