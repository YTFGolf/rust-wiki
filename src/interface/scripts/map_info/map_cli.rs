//! `map_info` command.

use crate::{
    config::Config,
    game_data::map::parsed::map::GameMap,
    interface::{
        cli::{
            base::BaseOptions,
            cli_util::input,
            cli_util::{CommandExec, ConfigMerge},
            version_opt::VersionOptions,
        },
        scripts::map_info::get_map_info,
    },
};
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Map info options.
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
            1 => self.selector[0].clone(),
            0 => input("Input selector: "),
            _ => self.selector.join(" "),
        };

        let map = GameMap::from_selector(&selector, config.version.current_version()).unwrap();
        let info = get_map_info(&map, config);
        println!("{info}");
    }
}
