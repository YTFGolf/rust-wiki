//! `cat` command.

use crate::interface::{
    cli::{
        base::BaseOptions,
        cli_util::{CommandExec, ConfigMerge},
        version_opt::VersionOptions,
    },
    config::{Config, cat_config::StatsTemplateVersion},
    scripts::cat_info::cat_info::get_info,
};
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct CatInfoOptions {
    /// Cat id.
    pub id: String,

    #[arg(long)]
    /// Use old template format.
    pub old: bool,

    #[command(flatten)]
    /// Global options.
    pub base: BaseOptions,
    #[command(flatten)]
    /// Version options.
    pub version: VersionOptions,
}
impl ConfigMerge for CatInfoOptions {
    fn merge(&self, config: &mut Config) {
        self.base.merge(config);
        self.version.merge(config);

        if self.old {
            config.cat_info.stats_template_version = StatsTemplateVersion::Manual;
        }
    }
}
impl CommandExec for CatInfoOptions {
    fn exec(&self, config: &Config) {
        let name_or_id = &self.id;
        // let id = match ENEMY_DATA.get_id_from_name(name_or_id) {
        let id = match None::<u32> {
            None => name_or_id.parse().unwrap(),
            Some(id) => id,
        };

        let info = get_info(id, config).unwrap();
        println!("{info}");
    }
}
