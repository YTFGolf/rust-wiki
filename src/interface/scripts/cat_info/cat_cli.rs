//! `cat` command.

use crate::{
    interface::{
        cli::{
            base::BaseOptions,
            cli_util::{CommandExec, ConfigMerge},
            version_opt::VersionOptions,
        },
        config::{Config, cat_config::StatsTemplateVersion},
        scripts::cat_info::cat_info::get_info,
    },
    wiki_data::cat_data::CAT_DATA,
};
use clap::Args;

#[derive(Debug, Args, PartialEq)]
/// Stage info options.
pub struct CatInfoOptions {
    /// Cat name or id.
    pub name_or_id: String,

    #[arg(long)]
    /// Stats template version to use.
    pub stats_version: Option<StatsTemplateVersion>,
    #[arg(long)]
    /// Use validation parameters on cat stats.
    pub use_stats_validation: Option<bool>,

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

        if let Some(v) = self.stats_version {
            config.cat_info.stats_template_version = v;
        }
        if let Some(v) = self.use_stats_validation {
            config.cat_info.stats_hide_validation = !v;
        }
    }
}
impl CommandExec for CatInfoOptions {
    fn exec(&self, config: &Config) {
        let id = match self.name_or_id.parse() {
            Ok(id) => id,
            Err(_) => match CAT_DATA.get_id_from_name(&self.name_or_id) {
                Some(id) => id,
                None => panic!(
                    "{e:?} is not a valid number or cat form name!",
                    e = self.name_or_id
                ),
            },
        };

        let info = get_info(id, config).unwrap();
        println!("{info}");
    }
}
