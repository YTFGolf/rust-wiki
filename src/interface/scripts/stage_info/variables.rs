//! Deals with stage info variables.

use super::{
    battlegrounds::battlegrounds,
    beginning::{enemies_appearing, intro},
    enemies_list::enemies_list,
    information::{base_hp, energy, max_enemies, stage_location, stage_name, width, xp},
    misc_information::{chapter, difficulty, max_clears, stage_nav, star},
    restrictions::{restrictions_info, restrictions_section, rules_section},
    stage_info::StageWikiDataContainer,
    treasure::{score_rewards, treasure},
};
use crate::{
    game_data::stage::parsed::stage::Stage,
    interface::{config::Config, scripts::stage_info::information::time_limit},
    wikitext::template::Template,
};

/// Default format for stage info.
pub const DEFAULT_FORMAT: &str = "\
${enemies_appearing}
${intro}

${si_template}

==Rules==
${rules}

==Restrictions==
${restrictions_section}

==Battleground==
${battlegrounds}

==Strategy==
-

==Reference==
*${reference}\
";

fn si_template(
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

/// Get the content of a format variable.
pub fn get_stage_variable(
    variable_name: &str,
    stage: &Stage,
    stage_wiki_data: &StageWikiDataContainer,
    config: &Config,
) -> String {
    match variable_name {
        "enemies_appearing" => enemies_appearing(stage),
        "intro" => intro(stage, stage_wiki_data),
        "si_template" => si_template(stage, stage_wiki_data, config).to_string(),
        "restrictions_section" => restrictions_section(stage).unwrap_or_default(),
        "rules" => rules_section(stage).unwrap_or_default(),
        "battlegrounds" => battlegrounds(stage),
        "reference" => reference(stage),

        invalid => panic!("Variable {invalid:?} is not recognised!"),
    }
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

// TODO tests
