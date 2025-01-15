//! Deals with the config for stage info.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StageConfig {
    suppress_gauntlet_mags: bool,
}
