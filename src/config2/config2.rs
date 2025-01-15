//! Represents the config.

use log::Level;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

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

fn deserialize_log<'de, D>(deserializer: D) -> Result<Level, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Level::from_str(s).map_err(serde::de::Error::custom)
}

fn serialize_log<S>(level: &Level, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&level.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
/// Configuration
// TODO remove Serialise and replace with toml-edit.
pub struct Config {
    #[serde(deserialize_with = "deserialize_log", serialize_with = "serialize_log")]
    log_level: Level,
    /// Username etc.
    wiki: (),
    /// Game version config.
    version: (),
    /// Config for `stage_info`.
    stage_info: (),
}
