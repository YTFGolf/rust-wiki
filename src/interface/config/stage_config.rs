//! Deals with the config for stage info.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
/// Config for stage info.
pub struct StageConfig {
    /// Do you suppress gauntlet mags.
    suppress_gauntlet_mags: bool,
}
impl StageConfig {
    /// Do you suppress gauntlet mags.
    pub fn suppress(&self) -> bool {
        self.suppress_gauntlet_mags
    }

    /// Set the suppress gauntlet mags flag.
    pub fn set_suppress(&mut self, value: bool) {
        self.suppress_gauntlet_mags = value;
    }
}
