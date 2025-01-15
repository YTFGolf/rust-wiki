//! Deals with the config for version.

use serde::{Deserialize, Serialize};

/// Main language.
#[allow(missing_docs)]
#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Lang {
    EN,
    #[default]
    JP,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VersionConfig {
    lang: Lang,
}

impl VersionConfig {}
