//! Deals with the config for wiki reader.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiConfig {
    username: String,
}
