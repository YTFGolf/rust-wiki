//! Root module for stage info script.

use crate::{
    game_data::{meta::stage::stage_id::StageID, stage::parsed::stage::Stage},
    interface::{
        config::Config,
        scripts::stage_info::{
            battlegrounds::battlegrounds,
            beginning::{enemies_appearing, intro},
            enemies_list::enemies_list,
            information::{
                base_hp, energy, max_enemies, stage_location, stage_name, time_limit, width, xp,
            },
            misc_information::{chapter, difficulty, max_clears, stage_nav, star},
            restrictions::{restrictions_info, restrictions_section, rules_section},
            treasure::{score_rewards, treasure},
        },
    },
    wiki_data::stage_wiki_data::{MapWikiData, STAGE_WIKI_DATA, StageWikiData},
    wikitext::{page::Page, section::Section, template::Template},
};
use std::fmt::Display;

/// Container for wiki data about a stage.
pub struct StageWikiDataContainer {
    // TODO rename
    /// Stage's map.
    pub stage_map: &'static MapWikiData,
    /// Stage itself.
    pub stage_name: &'static StageWikiData,
}

/// Stage info template.
pub fn si_template(
    stage: &Stage,
    stage_wiki_data: &StageWikiDataContainer,
    config: &Config,
) -> Template {
    Template::named("Stage Info")
        .add_params(stage_name(stage, config.version.lang()))
        .add_params(stage_location(stage, config.version.lang()))
        .add_params(energy(stage))
        .add_params(base_hp(stage))
        .add_params(enemies_list(stage, config.stage_info.suppress()))
        .add_params(treasure(stage))
        .add_params(restrictions_info(stage))
        .add_params(time_limit(stage))
        .add_params(score_rewards(stage))
        .add_params(xp(stage))
        .add_params(width(stage))
        .add_params(max_enemies(stage))
        .add_const(&[("jpname", "?"), ("script", "?"), ("romaji", "?")])
        .add_params(star(stage))
        .add_params(chapter(stage, stage_wiki_data))
        .add_params(max_clears(stage))
        .add_params(difficulty(stage))
        .add_params(stage_nav(stage, stage_wiki_data))
}

/// Get the battlecats-db reference link.
fn reference(stage: &Stage) -> String {
    format!(
        "https://battlecats-db.com/stage/s{type:02}{map:03}-{incremented_stage:02}.html",
        r#type = stage.id.variant().num(),
        map = stage.id.map().num(),
        incremented_stage = stage.id.num() + 1,
    )
}

/// Get full stage info.
pub fn get_stage_info(stage: &Stage, config: &Config) -> impl Display {
    let mut page = Page::blank();
    let stage_wiki_data = get_stage_wiki_data(&stage.id);

    let appears = enemies_appearing(stage);
    let opener = intro(stage, &stage_wiki_data);
    let intro_sect = Section::blank(appears + "\n" + &opener);
    page.push(intro_sect);

    page.push(Section::blank(
        si_template(stage, &stage_wiki_data, config).to_string(),
    ));

    if let Some(s) = rules_section(stage) {
        page.push(Section::h2("Rules", s));
    }
    if let Some(s) = restrictions_section(stage) {
        page.push(Section::h2("Restrictions", s));
    }

    page.push(Section::h2("Battlegrounds", battlegrounds(stage)));
    page.push(Section::h2("Strategy", "-"));
    page.push(Section::h2("Reference", reference(stage)));

    page
}

/// Get the stage's corresponding wiki data.
pub fn get_stage_wiki_data(stage: &StageID) -> StageWikiDataContainer {
    let stage_map = STAGE_WIKI_DATA
        .stage_map(stage.map())
        .unwrap_or_else(|| panic!("Couldn't find map name: {}", stage.map()));
    let stage_name = stage_map
        .get(stage.num())
        .unwrap_or_else(|| panic!("Couldn't find stage name: {stage}"));

    StageWikiDataContainer {
        stage_map,
        stage_name,
    }
}

// TODO tests
