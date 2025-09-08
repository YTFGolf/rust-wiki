//! Script for cat info.

use crate::{
    game_data::cat::parsed::cat::{Cat, CatDataError},
    interface::{
        config::{Config, cat_config::StatsTemplateVersion},
        scripts::cat_info::stats::template::{current::get_template, manual::get_old_template},
    },
};

/// Get cat info.
pub fn get_info(wiki_id: u32, config: &Config) -> Result<String, CatDataError> {
    let cat = Cat::from_wiki_id(wiki_id, &config.version)?;

    let template = match config.cat_info.stats_template_version {
        StatsTemplateVersion::Current => get_template(&cat).to_string(),
        StatsTemplateVersion::Manual => get_old_template(&cat).to_string(),
    };

    Ok(template)
}

/*
talents
combos
desc
*/
