//! `gauntlet` command.

use crate::{
    config::Config,
    game_data::meta::stage::stage_types::parse::parse_map::parse_general_map_id,
    interface::{
        cli::{
            base::BaseOptions,
            cli_util::{CommandExec, ConfigMerge, input},
            version_opt::VersionOptions,
        },
        scripts::gauntlet::gauntlet::map_gauntlet,
    },
};
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Gauntlet options.
pub struct GauntletOptions {
    /// Gauntlet map selector.
    pub selector: Vec<String>,

    #[command(flatten)]
    /// Global options.
    pub base: BaseOptions,
    #[command(flatten)]
    /// Version options.
    pub version: VersionOptions,
}
impl ConfigMerge for GauntletOptions {
    fn merge(&self, config: &mut Config) {
        self.base.merge(config);
        self.version.merge(config);
    }
}
impl CommandExec for GauntletOptions {
    fn exec(&self, config: &Config) {
        let selector = match self.selector.len() {
            1 => self.selector[0].clone(),
            0 => input("Input selector: "),
            _ => self.selector.join(" "),
        };

        let gauntlet_id = parse_general_map_id(&selector).unwrap();
        let info = map_gauntlet(&gauntlet_id, config);
        println!("{info}");
    }
}
