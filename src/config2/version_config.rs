//! Deals with the config for version.

use serde::{Deserialize, Serialize};

impl<'de> Deserialize<'de> for VersionConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = deserializer;
        todo!()
    }
}

#[derive(Debug, Serialize)]
pub struct VersionConfig {}
