//! Deals with the config for wiki reader.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WikiConfig {
    username: String,
}
