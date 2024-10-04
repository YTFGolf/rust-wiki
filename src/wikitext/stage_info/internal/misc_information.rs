use crate::{
    data::stage::{parsed::stage::Stage, stage_metadata::consts::StageTypeEnum as T},
    wikitext::{
        data_files::stage_page_data::STAGE_NAMES, stage_info::StageWikiData,
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
