//! Deals with the config for encounters.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
/// Config for encounters.
pub struct EncountersConfig {
    /// Do you use the encounters cache?
    pub use_cache: bool,
    /// Do you write to the encounters cache?
    pub write_cache: bool,
}

impl EncountersConfig {}
