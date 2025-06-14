use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::{config::Config, scripts::cat_info::stats::template::get_template},
};

/// Do thing.
pub fn do_thing(wiki_id: u32, config: &Config) {
    get_template(Cat::from_wiki_id(wiki_id, &config.version).unwrap());
}

/*
talents
combos
desc (will need to make other parts better)
*/
