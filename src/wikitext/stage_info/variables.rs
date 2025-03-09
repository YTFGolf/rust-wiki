//! Deals with stage info variables.

use super::battlegrounds::battlegrounds;
use super::beginning::{enemies_appearing, intro};
use super::enemies_list::enemies_list;
use super::information::{base_hp, energy, max_enemies, stage_location, stage_name, width, xp};
use super::misc_information::{chapter, difficulty, max_clears, stage_nav, star};
use super::restrictions::{restrictions_info, restrictions_section, rules};
use super::treasure::{score_rewards, treasure};
use super::StageWikiDataContainer;
use crate::config::Config;
use crate::data::stage::parsed::stage::Stage;

/// Default format for stage info.
pub const DEFAULT_FORMAT: &str = "\
${enemies_appearing}
${intro}

{{Stage Info
${stage_name}
${stage_location}
${energy}
${base_hp}
${enemies_list}
${treasure}
${restrictions_info}
${score_rewards}
${xp}
${width}
${max_enemies}
|jpname = ?\n|script = ?\n|romaji = ?
${star}
${chapter}
${max_clears}
${difficulty}
${stage_nav}
}}

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
        "stage_name" => stage_name(stage).to_string(),
        "stage_location" => stage_location(stage).to_string(),
        "energy" => energy(stage)
            .map(|param| param.to_string())
            .unwrap_or_default(),
        "base_hp" => base_hp(stage)
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        "enemies_list" => enemies_list(stage, config.stage_info.suppress())
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        "treasure" => treasure(stage)
            .map(|param| param.to_string())
            .unwrap_or_default(),
        "restrictions_info" => restrictions_info(stage)
            .map(|param| param.to_string())
            .unwrap_or_default(),
        "score_rewards" => score_rewards(stage)
            .map(|param| param.to_string())
            .unwrap_or_default(),
        "xp" => xp(stage).map(|param| param.to_string()).unwrap_or_default(),
        "width" => width(stage).to_string(),
        "max_enemies" => max_enemies(stage).to_string(),
        "star" => star(stage).to_string(),
        "chapter" => chapter(stage, stage_wiki_data)
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        "max_clears" => max_clears(stage)
            .map(|param| param.to_string())
            .unwrap_or_default(),
        "difficulty" => difficulty(stage)
            .map(|param| param.to_string())
            .unwrap_or_default(),
        "stage_nav" => stage_nav(stage, stage_wiki_data)
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
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
