use super::{
    base::BaseOptions,
    cli::{CommandExec, ConfigMerge},
    version_opt::VersionOptions,
};
use crate::{config::config::Config, wikitext::data_files::enemy_data::ENEMY_DATA};
use clap::{command, Args};

#[derive(Debug, Args, PartialEq)]
/// Encounters options.
pub struct EncountersOptions {
    /// Which units to get encounters for.
    pub names: Vec<String>,

    #[command(flatten)]
    /// Global options.
    pub base: BaseOptions,
    #[command(flatten)]
    /// Version options.
    pub version: VersionOptions,
}

impl ConfigMerge for EncountersOptions {
    fn merge(&self, config: &mut Config) {
        self.base.merge(config);
        self.version.merge(config);
    }
}

impl CommandExec for EncountersOptions {
    fn exec(&self, config: &Config) {
        log::warn!("This currently only works on the first enemy");
        let name_or_id = &self.names[0];
        let id = match ENEMY_DATA.get_id_from_name(name_or_id) {
            None => name_or_id.parse().unwrap(),
            Some(id) => *id,
        };

        crate::wikitext::encounters::do_thing(id, config);
    }
}
