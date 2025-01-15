//! Represents the command line interface.

use crate::config2::config2::Config;

/// Overwrite values of a [`Config`] object.
pub trait ConfigMerge {
    /// Overwrites values of `config` with equivalent values from `&self`.
    fn merge(&self, config: &mut Config);
}
