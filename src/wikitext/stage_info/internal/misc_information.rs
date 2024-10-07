use crate::{
    data::stage::{
        parsed::stage::{ContinueStages, Stage},
        stage_metadata::consts::StageTypeEnum as T,
    },
    wikitext::{
        data_files::stage_page_data::{MapData, StageData, STAGE_NAMES},
        stage_info::StageWikiData,
        template_parameter::TemplateParameter,
        wiki_utils::REGEXES,
    },
};
use regex::Regex;
use std::borrow::Cow;

pub fn star(stage: &Stage) -> TemplateParameter {
    let max_crowns: u8 = match stage.crown_data.as_ref() {
        Some(d) => d.max_difficulty.into(),
        None => 1,
    };

    TemplateParameter::new("star", max_crowns.to_string())
}

pub fn chapter(stage: &Stage, data: &StageWikiData) -> Vec<TemplateParameter> {
    #[inline]
    fn get_map_name(map: &MapData) -> Cow<'_, str> {
        REGEXES.old_or_removed_sub.replace_all(&map.name, "$1")
    }

    match stage.meta.type_enum {
        T::MainChapters => vec![],
        T::SoL | T::UL | T::ZL => vec![TemplateParameter::new(
            "sub-chapter",
            get_map_name(data.stage_map).to_string(),
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
                TemplateParameter::new("event-chapter", get_map_name(data.stage_map).to_string()),
            ]
        }
        _ => vec![TemplateParameter::new(
            "event-chapter",
            get_map_name(data.stage_map).to_string(),
        )],
    }
}

pub fn max_clears(stage: &Stage) -> Option<TemplateParameter> {
    Some(TemplateParameter::new(
        "max clears",
        stage.max_clears?.to_string(),
    ))
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
    assert!(
        location.is_none()
            || matches!(
                REGEXES
                    .old_or_removed_sub
                    .replace_all(&location.unwrap().name, "$1"),
                Cow::Borrowed(_)
            ),
        "Debug assert: navigation function."
    );
    match location {
        None => "N/A".to_string(),
        Some(location) => location.name.clone(), // Some(location) => REGEXES
                                                 //     .old_or_removed_sub
                                                 //     .replace_all(&location.name, "$1")
                                                 //     .to_string(),
    }
}

fn get_continuation_stages(data: &ContinueStages) -> String {
    let map = STAGE_NAMES
        .stage_map(4, data.map_id)
        .unwrap_or_else(|| panic!("Extra stages map with id {} was not found!", data.map_id));
    let stage_names = (data.stage_ids.0..data.stage_ids.1 + 1).map(|id| {
        let stage = &map.get(id).unwrap().name;

        match data.chance {
            100 => format!("{stage} (''Continuation Stage'')"),
            chance => {
                let single_cont_chance: f64 =
                    f64::from(chance) / f64::from(data.stage_ids.1 - data.stage_ids.0 + 1);
                let precision = if single_cont_chance % 1.0 == 0.0 {
                    0
                } else {
                    1
                };
                format!(
                    "{stage} (''Continuation Stage'', {single_cont_chance:.0$}%)",
                    precision
                )
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
        prev = if stage.meta.stage_num == 0 {
            None
        } else {
            data.stage_map.get(stage.meta.stage_num - 1)
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wikitext::stage_info::internal::test_util::get_stage_wiki_data;

    #[test]
    fn test_single_stage() {
        let earthshaker = Stage::new("n 0 0").unwrap();
        let data = get_stage_wiki_data(&earthshaker);
        assert_eq!(
            star(&earthshaker),
            TemplateParameter::new("star", "4".to_string())
        );
        assert_eq!(
            chapter(&earthshaker, &data),
            vec![TemplateParameter::new(
                "sub-chapter",
                "[[The Legend Begins]]".to_string()
            )]
        );
        assert_eq!(max_clears(&earthshaker), None);
        assert_eq!(
            difficulty(&earthshaker),
            Some(TemplateParameter::new("difficulty", "★1".to_string()))
        );
        assert_eq!(
            stage_nav(&earthshaker, &data),
            vec![
                TemplateParameter::new("prev stage", "N/A".to_string()),
                TemplateParameter::new("next stage", "[[Return of Terror]]".to_string())
            ]
        );
    }

    #[test]
    fn test_old_map() {
        let athletic_meet = Stage::new("s 8 0").unwrap();
        let data = get_stage_wiki_data(&athletic_meet);
        assert_eq!(
            data.stage_map.name,
            "[[Autumn = Sports Day! (Monthly Event)#Autumn Sports Day|Autumn Sports Day]] (Removed)"
        );
        assert_eq!(
            chapter(&athletic_meet, &data),
            vec![TemplateParameter::new(
                "event-chapter",
                "[[Autumn = Sports Day! (Monthly Event)#Autumn Sports Day|Autumn Sports Day]]"
                    .to_string()
            )]
        );
    }

    #[test]
    fn test_max_clears() {
        let steel_visage = Stage::new("s 78 0").unwrap();
        assert_eq!(
            max_clears(&steel_visage),
            Some(TemplateParameter::new("max clears", "1".to_string()))
        )
    }

    #[test]
    fn test_conditional_continue_single() {
        let spectrum_of_truth = Stage::new("s 222 0").unwrap();
        let data = get_stage_wiki_data(&spectrum_of_truth);
        assert_eq!(
            stage_nav(&spectrum_of_truth, &data),
            vec![
                TemplateParameter::new("prev stage", "N/A".to_string()),
                TemplateParameter::new(
                    "next stage",
                    "[[Miracle Iris (Deadly)]] (''Continuation Stage'', 40%)".to_string()
                ),
            ]
        )
    }

    #[test]
    fn test_conditional_continue_multiple() {
        let green_envy_3 = Stage::new("s 97 2").unwrap();
        let data = get_stage_wiki_data(&green_envy_3);
        assert_eq!(
            stage_nav(&green_envy_3, &data),
            vec![
                TemplateParameter::new("prev stage", "[[Green Envy (Expert)]]".to_string()),
                TemplateParameter::new(
                    "next stage",
                    "[[Catfruit Jubilee]] (''Continuation Stage'', 5%)<br>\n\
                    [[Catfruit Jubilee]] (''Continuation Stage'', 5%)<br>\n\
                    [[Catfruit Jubilee]] (''Continuation Stage'', 5%)"
                        .to_string()
                )
            ]
        );
    }

    #[test]
    fn test_continue_stage_nav() {
        let proving_grounds = Stage::new("s 250 2").unwrap();
        let data = get_stage_wiki_data(&proving_grounds);
        assert_eq!(
            max_clears(&proving_grounds),
            Some(TemplateParameter::new("max clears", "1".to_string()))
        );
        assert_eq!(
            stage_nav(&proving_grounds, &data),
            vec![
                TemplateParameter::new("prev stage", "[[First Round (Expert)]]".to_string()),
                TemplateParameter::new(
                    "next stage",
                    "[[2nd Round: Dawn (Deadly)]] (''Continuation Stage'')<br>\n\
                    [[2nd Round: Dusk (Deadly)]] (''Continuation Stage'')"
                        .to_string()
                )
            ]
        )
    }
}
