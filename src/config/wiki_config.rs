//! Deals with the config for wiki reader.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
/// Config for interacting with wiki.
pub struct WikiConfig {
    /// Wiki username.
    pub username: String,
}
