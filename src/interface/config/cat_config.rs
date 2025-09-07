//! Deals with the config for cat info.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
/// Config for cat info.
pub struct CatConfig {
    pub use_old_template: bool,
    // TODO manual/old flags
}
