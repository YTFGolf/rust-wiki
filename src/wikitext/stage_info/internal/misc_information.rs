use crate::{
    data::stage::{
        parsed::stage::{ContinueStages, Stage},
        stage_metadata::consts::StageTypeEnum as T,
    },
    wikitext::{
        data_files::stage_page_data::{StageData, STAGE_NAMES},
        stage_info::StageWikiData,
        template_parameter::TemplateParameter,
    },
};
use regex::Regex;

pub fn star(stage: &Stage) -> TemplateParameter {
    let max_crowns: u8 = match stage.crown_data.as_ref() {
        Some(d) => d.max_difficulty.into(),
        None => 1,
    };

    TemplateParameter::new("star", max_crowns.to_string())
}

pub fn chapter(stage: &Stage, data: &StageWikiData) -> Vec<TemplateParameter> {
    match stage.meta.type_enum {
        T::MainChapters => vec![],
        T::SoL | T::UL | T::ZL => vec![TemplateParameter::new(
            "sub-chapter",
            data.stage_map.name.clone(),
        )],
        // TODO use a cow string instead of clone
        T::Collab | T::CollabGauntlet => {
            let collab_name = Regex::new(r"\[\[(.*? Event)").unwrap();
            let collab_name = match collab_name.captures_iter(&data.stage_map.name).next() {
                Some(c) => c.get(1).unwrap().as_str(),
                None => "name",
            };

            let event = TemplateParameter::new("event", format!("[[{collab_name}]]"));
            vec![
                event,
                TemplateParameter::new("event-chapter", data.stage_map.name.clone()),
            ]
        }
        _ => vec![TemplateParameter::new(
            "event-chapter",
            data.stage_map.name.clone(),
        )],
    }
}

pub fn difficulty(stage: &Stage) -> Option<TemplateParameter> {
    let difficulty = STAGE_NAMES.difficulty(
        stage.meta.type_num,
        stage.meta.map_num,
        stage.meta.stage_num,
    )?;

    Some(TemplateParameter::new(
        "difficulty",
        format!("★{difficulty}"),
    ))
}

fn get_single_nav(location: Option<&StageData>) -> String {
    match location {
        None => "N/A".to_string(),
        Some(location) => location.name.clone(),
    }
}

fn get_continuation_stages(data: &ContinueStages) -> String {
    let map = STAGE_NAMES
        .stage_map(4, data.map_id)
        .unwrap_or_else(|| panic!("Extra stages map with id {} was not found!", data.map_id));
    let stage_names = (data.stage_ids.0 .. data.stage_ids.1 + 1).map(|id| {
        let stage = &map.get(id).unwrap().name;

        match data.chance {
            100 => format!("{stage} (''Continuation Stage'')"),
            chance => {
                assert_eq!(data.stage_ids.0, data.stage_ids.1, "Feature currently not supported: non-guaranteed continuation stage with multiple possible stages to continue to.");
                format!("{stage} (''Continuation Stage'', {chance}%)")
            }
        }
    });

    stage_names.collect::<Vec<String>>().join("<br>\n")
}

fn get_nav(stage: &Stage, data: &StageWikiData) -> (String, String) {
    let prev;
    let next;
    if [T::Extra].contains(&stage.meta.type_enum) {
        prev = None;
        next = None
    } else {
        prev = data.stage_map.get(stage.meta.stage_num - 1);
        next = data.stage_map.get(stage.meta.stage_num + 1);
    }

    let continue_data = match stage.continue_data.as_ref() {
        None => return (get_single_nav(prev), get_single_nav(next)),
        Some(data) => data,
    };

    let prev_str = get_single_nav(prev);
    let next_str = match next {
        None => get_continuation_stages(continue_data),
        Some(_) => get_single_nav(next) + "<br>\n" + &get_continuation_stages(continue_data),
    };

    (prev_str, next_str)
}

pub fn stage_nav(stage: &Stage, data: &StageWikiData) -> Vec<TemplateParameter> {
    if [T::Dojo, T::RankingDojo].contains(&stage.meta.type_enum) {
        return vec![];
    }

    let (prev, next) = get_nav(stage, data);
    vec![
        TemplateParameter::new("prev stage", prev),
        TemplateParameter::new("next stage", next),
    ]
}

// proving grounds
