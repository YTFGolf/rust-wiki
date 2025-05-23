//! `cat` command.

use super::{
    base::BaseOptions,
    cli_util::{CommandExec, ConfigMerge},
    version_opt::VersionOptions,
};
use crate::config::Config;
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct CatInfoOptions {
    /// Cat id.
    pub id: String,

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

        crate::wikitext::cat_info::do_thing(id, config);
    }
}
