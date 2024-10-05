//! Prints information about a stage.

mod internal;
use super::data_files::stage_page_data::{MapData, StageData, STAGE_NAMES};
use super::format_parser::{parse_si_format, ParseType};
use crate::data::stage::parsed::stage::Stage;
use regex::Regex;
use std::fmt::Write;

const DEFAULT_FORMAT: &str = "\
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
${difficulty}
${stage_nav}
}}

==Restrictions==
${restrictions_section}

==Battleground==
${battlegrounds}

==Strategy==
-

==Reference==
*${reference}\
";

struct StageWikiData {
    stage_map: &'static MapData,
    stage_name: &'static StageData,
}

pub fn get_stage_info(stage: &Stage)->String{
    get_stage_info_formatted(stage, DEFAULT_FORMAT)
}

pub fn get_stage_info_formatted(stage: &Stage, format: &str) -> String {
    // TODO pre and post probably
    let parsed = parse_si_format(format);

    let mut buf = "".to_string();

    let stage_map = STAGE_NAMES
        .stage_map(stage.meta.type_num, stage.meta.map_num)
        .unwrap();
    let stage_name = stage_map.get(stage.meta.stage_num).unwrap();

    let stage_wiki_data = StageWikiData {
        stage_map,
        stage_name,
    };

    for node in parsed {
        if node.ptype == ParseType::Text {
            buf.write_str(node.content).unwrap();
            continue;
        }

        let new_buf = get_stage_variable(node.content, &stage, &stage_wiki_data);
        buf.write_str(&new_buf).unwrap();
    }

    let buf = Regex::new(r"\n+(\||\}\})")
        .unwrap()
        .replace_all(&buf, "\n$1");
    let buf = Regex::new(r"\n==.*==\n\n").unwrap().replace_all(&buf, "");

    buf.to_string()
}

fn get_stage_variable(
    variable_name: &str,
    stage: &Stage,
    stage_wiki_data: &StageWikiData,
) -> String {
    match variable_name {
        "enemies_appearing" => internal::enemies_appearing(stage),
        "intro" => internal::intro(stage, stage_wiki_data),
        "stage_name" => internal::stage_name(stage).to_string(),
        "stage_location" => internal::stage_location(stage).to_string(),
        "energy" => internal::energy(stage)
            .map(|param| param.to_string())
            .unwrap_or("".to_string()),
        "base_hp" => internal::base_hp(stage)
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        "enemies_list" => internal::enemies_list(stage)
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        "treasure" => internal::treasure(stage)
            .map(|param| param.to_string())
            .unwrap_or("".to_string()),
        "restrictions_info" => internal::restrictions_info(stage)
            .map(|param| param.to_string())
            .unwrap_or("".to_string()),
        "score_rewards" => internal::score_rewards(stage)
            .map(|param| param.to_string())
            .unwrap_or("".to_string()),
        "xp" => internal::xp(stage)
            .map(|param| param.to_string())
            .unwrap_or("".to_string()),
        "width" => internal::width(stage).to_string(),
        "max_enemies" => internal::max_enemies(stage).to_string(),
        "star" => internal::star(stage).to_string(),
        "chapter" => internal::chapter(stage, stage_wiki_data)
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        "difficulty" => internal::difficulty(stage)
            .map(|param| param.to_string())
            .unwrap_or("".to_string()),
        "stage_nav" => internal::stage_nav(stage, stage_wiki_data)
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        "restrictions_section" => internal::restrictions_section(stage),
        "battlegrounds" => internal::battlegrounds(stage),
        "reference" => internal::reference(stage),

        invalid => panic!("Variable {invalid:?} is not recognised!"),
    }
}
