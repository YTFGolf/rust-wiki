//! Deals with stage info variables.

use super::StageWikiDataContainer;
use super::battlegrounds::battlegrounds;
use super::beginning::{enemies_appearing, intro};
use super::enemies_list::enemies_list;
use super::information::{base_hp, energy, max_enemies, stage_location, stage_name, width, xp};
use super::misc_information::{chapter, difficulty, max_clears, stage_nav, star};
use super::restrictions::{restrictions_info, restrictions_section, rules};
use super::treasure::{score_rewards, treasure};
use crate::config::Config;
use crate::data::stage::parsed::stage::Stage;
use crate::wikitext::template_parameter::Template;

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
    let mut t = Template::named("Stage Info")
        .add_params(stage_name(stage, config.version.lang()))
        .add_params(stage_location(stage, config.version.lang()))
        .add_params(energy(stage))
        .add_params(base_hp(stage))
        .add_params(enemies_list(stage, config.stage_info.suppress()))
        .add_params(treasure(stage))
        .add_params(restrictions_info(stage))
        .add_params(score_rewards(stage))
        .add_params(xp(stage))
        .add_params(width(stage))
        .add_params(max_enemies(stage))
        .add_const(&[("jpname", "?"), ("script", "?"), ("romaji", "?")])
        .add_params(star(stage))
        .add_params(chapter(stage, stage_wiki_data))
        .add_params(max_clears(stage))
        .add_params(difficulty(stage))
        .add_params(stage_nav(stage, stage_wiki_data));

    t
}

/// Get the content of a format
/// [`Variable`][super::super::format_parser::ParseType::Variable].
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
        "restrictions_section" => restrictions_section(stage),
        "rules" => rules(stage),
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
