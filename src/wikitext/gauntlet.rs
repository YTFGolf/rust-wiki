#![allow(missing_docs)]

use crate::{
    config::Config,
    data::stage::parsed::stage::Stage,
    meta::stage::{map_id::MapID, stage_id::StageID, variant::StageVariantID as T},
    wikitext::stage_info::get_stage_wiki_data,
};

use super::{
    stage_info::{
        StageWikiDataContainer,
        enemies_list::enemies_list,
        information::{max_enemies, stage_location, stage_name, width},
        misc_information::{chapter, max_clears, star},
        restrictions::restrictions_info,
    },
    template::Template,
};

fn template_check(
    stage: &Stage,
    stage_wiki_data: &StageWikiDataContainer,
    config: &Config,
) -> Template {
    Template::named("Stage Info")
        // .add_params(stage_name(stage, config.version.lang()))
        // .add_params(stage_location(stage, config.version.lang()))
        // .add_params(energy(stage))
        // .add_params(base_hp(stage))
        .add_params(enemies_list(stage, true))
        // .add_params(treasure(stage))
        .add_params(restrictions_info(stage))
        // .add_params(score_rewards(stage))
        // .add_params(xp(stage))
        .add_params(width(stage))
        .add_params(max_enemies(stage))
        .add_const(&[("jpname", "?"), ("script", "?"), ("romaji", "?")])
        .add_params(star(stage))
        .add_params(chapter(stage, stage_wiki_data))
        .add_params(max_clears(stage))
    // .add_params(difficulty(stage))
    // .add_params(stage_nav(stage, stage_wiki_data))
}

pub fn do_thing(config: &Config) {
    let map_id = MapID::from_components(T::Gauntlet, 0);
    for i in 0..20 {
        let id = StageID::from_map(map_id.clone(), i);
        let gauntlet = Stage::from_id(id, config.version.current_version()).unwrap();
        let data = get_stage_wiki_data(&gauntlet.id);
    }
    panic!("End")
}
