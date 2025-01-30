//! Deals with the config for map info.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
/// Config for map info.
pub struct MapConfig {
    display_version: bool,
}
impl MapConfig {
    /// Do you display current version.
    pub fn version(&self) -> bool {
        self.display_version
    }

    /// Set the version display flag.
    pub fn set_version(&mut self, value: bool) {
        self.display_version = value;
    }
}
