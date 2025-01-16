//! Config values related to the version of the game being used.

use super::cli::ConfigMerge;
use crate::config::Config;
use clap::Args;

#[derive(Debug, Default, Args, PartialEq)]
/// Options that can apply to every submodule.
pub struct VersionOptions {
    #[arg(short, long)]
    /// Root directory of decrypted files.
    path: Option<String>,
    // probably do lang as well
}
impl ConfigMerge for VersionOptions {
    fn merge(&self, config: &mut Config) {
        let version = &mut config.version;
        if let Some(path) = &self.path {
            version.set_current_path(path.clone());
        }

        version.init_all();
    }
}
