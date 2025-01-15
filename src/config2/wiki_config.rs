//! Deals with the config for wiki reader.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
/// Config for interacting with wiki.
pub struct WikiConfig {
    /// Wiki username.
    pub username: String,
}
