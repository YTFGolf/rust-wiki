//! Deals with the config for cat info.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
/// Which version of the stats template to use.
pub enum StatsTemplateVersion {
    #[default]
    /// Latest version.
    Current,
    /// Manual template.
    Manual,
    // /// Version 0.1.
    // Ver0o1
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
/// Config for cat info.
pub struct CatConfig {
    /// Which version of stats template to use.
    pub stats_template_version: StatsTemplateVersion,
}
