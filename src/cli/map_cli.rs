//! `map_info` command.

use super::{
    base::BaseOptions,
    cli_util::{CommandExec, ConfigMerge},
    version_opt::VersionOptions,
};
use crate::config::Config;
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct MapInfoOptions {
    #[command(flatten)]
    /// Global options.
    pub base: BaseOptions,
    #[command(flatten)]
    /// Version options.
    pub version: VersionOptions,
}
impl ConfigMerge for MapInfoOptions {
    fn merge(&self, config: &mut Config) {
        self.base.merge(config);
        self.version.merge(config);
    }
}
impl CommandExec for MapInfoOptions {
    fn exec(&self, config: &Config) {
        todo!("{config:#?}")
        // let selector = match self.selector.len() {
        //     1 => &self.selector[0],
        //     0 => &input("Input file selector: "),
        //     _ => &self.selector.join(" "),
        // };
        // println!(
        //     "{}",
        //     get_stage_info(
        //         &Stage::new(selector, config.version.current_version()).unwrap(),
        //         config
        //     )
        // );
    }
}
