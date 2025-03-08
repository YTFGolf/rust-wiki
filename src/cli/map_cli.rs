//! `map_info` command.

use super::{
    base::BaseOptions,
    cli_util::{CommandExec, ConfigMerge},
    version_opt::VersionOptions,
};
use crate::{
    cli::cli_util::input, config::Config, data::map::parsed::map::MapData,
    wikitext::map_info::get_map_info,
};
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct MapInfoOptions {
    /// Map selector.
    pub selector: Vec<String>,

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
        let selector = match self.selector.len() {
            1 => self.selector[0].to_string(),
            0 => input("Input selector: "),
            _ => self.selector.join(" "),
        };

        let map = MapData::from_selector(&selector, config.version.current_version()).unwrap();
        let info = get_map_info(&map, config);
        println!("{info}");
    }
}
