//! Gets the misc information of the stage infobox.

use crate::{
    data::stage::{
        parsed::stage::{ContinueStages, Stage},
        raw::stage_metadata::consts::StageTypeEnum as T,
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

/// Get the max crown difficulty of a stage.
pub fn star(stage: &Stage) -> TemplateParameter {
    let max_crowns: u8 = match stage.crown_data.as_ref() {
        Some(d) => d.max_difficulty.into(),
        None => 1,
    };

    TemplateParameter::new("star", max_crowns.to_string())
}

/// Get the `event`, `event-chapter` or `sub-chapter` items.
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
        T::Dojo | T::RankingDojo | T::NewType => vec![TemplateParameter::new(
            "dojo-chapter",
            get_map_name(data.stage_map).to_string(),
        )],
        _ => vec![TemplateParameter::new(
            "event-chapter",
            get_map_name(data.stage_map).to_string(),
        )],
    }
}

/// Get max clears of stage.
pub fn max_clears(stage: &Stage) -> Option<TemplateParameter> {
    Some(TemplateParameter::new(
        "max clears",
        stage.max_clears?.to_string(),
    ))
}

/// Get star difficulty of stage.
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

/// Get a single possible next or prev stage's string representation.
fn get_single_nav(location: Option<&StageData>) -> String {
    assert!(
        location.is_none()
            || matches!(
                REGEXES
                    .old_or_removed_sub
                    .replace_all(&location.unwrap().name, "$1"),
                Cow::Borrowed(_)
            ),
        "Debug assert: stage has (Old) or (Removed) and needs to be added to \
        test suite."
    );
    match location {
        None => "N/A".to_string(),
        Some(location) => location.name.clone(), // Some(location) => REGEXES
                                                 //     .old_or_removed_sub
                                                 //     .replace_all(&location.name, "$1")
                                                 //     .to_string(),
    }
}

/// Get all continuation stages possible from current stage.
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

/// Get the prev and next stage nav items.
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

    let (prev, mut next) = (get_single_nav(prev), get_single_nav(next));

    /// Add in additional items to navigation
    fn merge_nav(current: String, additional: String) -> String {
        match current.as_str() {
            "" | "N/A" => additional,
            _ => current + "<br>\n" + &additional,
        }
    }

    if let Some(ex_map_id) = stage.ex_invasion {
        let stage = &STAGE_NAMES
            .stage(4, ex_map_id % 1000, stage.meta.stage_num)
            .unwrap()
            .name;
        let invaded = format!("{stage} (''Invasion Stage'')");
        next = merge_nav(next, invaded);
    }

    if let Some(continue_data) = stage.continue_data.as_ref() {
        next = merge_nav(next, get_continuation_stages(continue_data));
    };

    (prev, next)
}

/// Get the `prev stage` and `next stage` infobox parameters.
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
        let earthshaker = Stage::new_current("n 0 0").unwrap();
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
    fn test_dojo() {
        let wanderer = Stage::new_current("dojo 0 0").unwrap();
        let data = get_stage_wiki_data(&wanderer);
        assert_eq!(
            chapter(&wanderer, &data),
            vec![TemplateParameter::new(
                "dojo-chapter",
                "[[Catclaw Dojo|Hall of Initiates]]".to_string()
            )]
        );

        let crimson_trial_arena = Stage::new_current("rank 0 0").unwrap();
        let data = get_stage_wiki_data(&crimson_trial_arena);
        assert_eq!(
            chapter(&crimson_trial_arena, &data),
            vec![TemplateParameter::new(
                "dojo-chapter",
                "[[Arena of Honor]]".to_string()
            )]
        );

        let 昇段試験1 = Stage::new_current("g 0 0").unwrap();
        let data = get_stage_wiki_data(&昇段試験1);
        assert_eq!(
            chapter(&昇段試験1, &data),
            vec![TemplateParameter::new(
                "dojo-chapter",
                "[[にゃんこ道検定#にゃんこ道検定 初段|にゃんこ道検定 初段]]".to_string()
            )]
        );
    }

    #[test]
    fn test_old_map() {
        let athletic_meet = Stage::new_current("s 8 0").unwrap();
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
        let steel_visage = Stage::new_current("s 78 0").unwrap();
        assert_eq!(
            max_clears(&steel_visage),
            Some(TemplateParameter::new("max clears", "1".to_string()))
        )
    }

    #[test]
    fn test_conditional_continue_single() {
        let spectrum_of_truth = Stage::new_current("s 222 0").unwrap();
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
        let green_envy_3 = Stage::new_current("s 97 2").unwrap();
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
        let proving_grounds = Stage::new_current("s 250 2").unwrap();
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

    #[test]
    fn test_ex_invasion() {
        let proving_grounds = Stage::new_current("s 385 0").unwrap();
        todo!()
    }
}
