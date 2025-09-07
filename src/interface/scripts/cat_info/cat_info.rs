//! Script for cat info.

use crate::{
    game_data::cat::parsed::cat::{Cat, CatDataError},
    interface::{
        config::Config,
        scripts::cat_info::stats::{template::get_template, template_manual::get_old_template},
    },
};

/// Get cat info.
pub fn get_info(wiki_id: u32, config: &Config) -> Result<String, CatDataError> {
    let cat = Cat::from_wiki_id(wiki_id, &config.version)?;

    let template;
    if config.cat_info.use_old_template {
        template = get_old_template(&cat).to_string();
    } else {
        template = get_template(&cat).to_string();
    }

    Ok(template)
}

/*
talents
combos
desc
*/
