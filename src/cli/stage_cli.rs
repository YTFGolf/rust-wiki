//     Command::StageInfo(si) => {
//         let config = &get_config(config, si.config.clone());
//         stage_info(si, config);
//     }

use super::{
    base::BaseOptions,
    cli::{CommandExec, ConfigMerge},
    version_opt::VersionOptions,
};
use crate::{
    cli::cli::input, config2::config2::Config, data::stage::parsed::stage::Stage,
    wikitext::stage_info::get_stage_info,
};
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
    #[command(flatten)]
    /// Version options.
    pub version: VersionOptions,
}
impl ConfigMerge for StageInfoOptions {
    fn merge(&self, config: &mut Config) {
        self.base.merge(config);
        self.version.merge(config);

        let info = &mut config.stage_info;
        if let Some(suppress) = self.suppress {
            info.set_suppress(suppress);
        }
    }
}
impl CommandExec for StageInfoOptions {
    fn exec(&self, config: &Config) {
        let selector = match self.selector.len() {
            1 => &self.selector[0],
            0 => &input("Input file selector: "),
            _ => &self.selector.join(" "),
        };
        println!(
            "{}",
            get_stage_info(
                &Stage::new(selector, config.version.current_version()).unwrap(),
                config
            )
        );
    }
}
