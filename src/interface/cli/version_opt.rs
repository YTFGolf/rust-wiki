//! Config values related to the version of the game being used.

use super::cli_util::ConfigMerge;
use crate::interface::config::Config;
use clap::Args;

#[derive(Debug, Default, Args, PartialEq)]
/// Options that can apply to every submodule.
pub struct VersionOptions {
    #[arg(short, long)]
    /// Root directory of decrypted files.
    path: Option<String>,

    #[arg(long)]
    /// Language. Use EN or JP or TW or KR or Fallback or actually fix this for
    /// the love of god.
    lang: Option<String>,
    // TODO make an enum
}
impl ConfigMerge for VersionOptions {
    fn merge(&self, config: &mut Config) {
        let version = &mut config.version;

        if let Some(lang) = &self.lang {
            version.set_lang(lang.parse().unwrap());
        }

        if let Some(path) = &self.path {
            version.set_current_path(path.clone());
        }

        version.init_all();
    }
}
