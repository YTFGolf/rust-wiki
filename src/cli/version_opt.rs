use super::cli::ConfigMerge;
use crate::config2::config2::Config;
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Options that can apply to every submodule.
pub struct VersionOptions {
    #[arg(short, long)]
    path: Option<String>,
    // probably do lang as well
}
impl ConfigMerge for VersionOptions {
    fn merge(&self, config: &mut Config) {
        let version = &mut config.version;
        if let Some(path) = &self.path {
            version.set_current_path(path.to_string());
        }

        version.init_all();
    }
}
