//     Command::StageInfo(si) => {
//         let config = &get_config(config, si.config.clone());
//         stage_info(si, config);
//     }

use super::{
    base::BaseOptions,
    cli::{CliCommand, CommandExec, ConfigMerge},
};
use crate::config2::config2::Config;
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct StageInfoOptions {
    /// Stage selector.
    pub selector: Vec<String>,

    #[arg(long)]
    /// Do you put `|0` in the Magnification template instead of the actual
    /// magnification for gauntlets?
    pub suppress: Option<bool>,

    #[command(flatten)]
    /// Global options.
    pub base: BaseOptions,
}
impl ConfigMerge for StageInfoOptions {
    fn merge(&self, config: &mut Config) {
        self.base.merge(config);

        let info = &mut config.stage_info;
        if let Some(suppress) = self.suppress {
            info.set_suppress(suppress);
        }
    }
}
impl CommandExec for StageInfoOptions {
    fn exec(&self, config: &Config) {
        todo!()
    }
}
