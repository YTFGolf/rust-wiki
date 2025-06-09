//! Gets the misc information of the stage infobox.

use super::stage_info::StageWikiDataContainer;
use crate::{
    game_data::{
        meta::stage::{map_id::MapID, stage_id::StageID, variant::StageVariantID as T},
        stage::parsed::stage::{ContinueStages, Stage},
    },
    regex_handler::static_regex,
    wiki_data::stage_wiki_data::{MapWikiData, STAGE_WIKI_DATA, StageWikiData},
    wikitext::{
        number_utils::get_formatted_float, template::TemplateParameter,
        text_utils::OLD_OR_REMOVED_SUB,
    },
};
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
pub fn chapter(stage: &Stage, data: &StageWikiDataContainer) -> Vec<TemplateParameter> {
    fn get_map_name(map: &MapWikiData) -> String {
        OLD_OR_REMOVED_SUB.replace_all(&map.name, "$1").into_owned()
    }

    match stage.id.variant() {
        T::MainChapters
        | T::EocOutbreak
        | T::ItfOutbreak
        | T::CotcOutbreak
        | T::Filibuster
        | T::AkuRealms => vec![],
        T::SoL | T::UL | T::ZL => vec![TemplateParameter::new(
            "sub-chapter",
            get_map_name(data.stage_map),
        )],
        T::Collab | T::CollabGauntlet => {
            let collab_name = static_regex(r"\[\[(.*? Event)");
            let collab_name = match collab_name.captures_iter(&data.stage_map.name).next() {
                Some(c) => c.get(1).unwrap().as_str(),
                None => "name",
            };

            let event = TemplateParameter::new("event", format!("[[{collab_name}]]"));
            vec![
                event,
                TemplateParameter::new("event-chapter", get_map_name(data.stage_map)),
            ]
        }
        T::Dojo | T::RankingDojo | T::Championships => vec![TemplateParameter::new(
            "dojo-chapter",
            get_map_name(data.stage_map),
        )],
        T::Event
        | T::Extra
        | T::Tower
        | T::Challenge
        | T::Catamin
        | T::Gauntlet
        | T::Enigma
        | T::Behemoth
        | T::Labyrinth
        | T::Colosseum => vec![TemplateParameter::new(
            "event-chapter",
            get_map_name(data.stage_map),
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
    let difficulty = STAGE_WIKI_DATA.difficulty(&stage.id)?;

    Some(TemplateParameter::new(
        "difficulty",
        format!("★{difficulty}"),
    ))
}

/// Get a single possible next or prev stage's string representation.
fn get_single_nav(location: Option<&StageWikiData>) -> String {
    assert!(
        location.is_none()
            || matches!(
                OLD_OR_REMOVED_SUB.replace_all(&location.unwrap().name, "$1"),
                Cow::Borrowed(_)
            ),
        "Debug assert: stage has (Old) or (Removed) and needs to be added to \
        test suite."
    );
    match location {
        None => "N/A".to_string(),
        Some(location) => location.name.clone(), // Some(location) => OLD_OR_REMOVED_SUB
                                                 //     .replace_all(&location.name, "$1")
                                                 //     .to_string(),
    }
}

/// Get all continuation stages possible from current stage.
fn get_continuation_stages(data: &ContinueStages) -> String {
    let map_id: MapID = MapID::from_numbers(4, data.map_id);
    let map = STAGE_WIKI_DATA
        .stage_map(&map_id)
        .unwrap_or_else(|| panic!("Extra stages map with id {} was not found!", data.map_id));
    let stage_names = (data.stage_ids.0..=data.stage_ids.1).map(|id| {
        let stage = &map.get(id).unwrap().name;

        match data.chance {
            100 => format!("{stage} (''Continuation Stage'')"),
            chance => {
                let single_cont_chance: f64 =
                    f64::from(chance) / f64::from(data.stage_ids.1 - data.stage_ids.0 + 1);
                let chance_repr = get_formatted_float(single_cont_chance, 1);
                format!("{stage} (''Continuation Stage'', {chance_repr}%)")
            }
        }
    });

    stage_names.collect::<Vec<String>>().join("<br>\n")
}

/// Get the prev and next stage nav items.
fn get_nav(stage: &Stage, data: &StageWikiDataContainer) -> (String, String) {
    let prev;
    let next;
    if [T::Extra].contains(&stage.id.variant()) {
        prev = None;
        next = None;
    } else {
        prev = if stage.id.num() == 0 {
            None
        } else {
            data.stage_map.get(stage.id.num() - 1)
        };
        next = data.stage_map.get(stage.id.num() + 1);
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
        let stage = &STAGE_WIKI_DATA
            .stage(&StageID::from_numbers(4, ex_map_id % 1000, stage.id.num()))
            .unwrap()
            .name;
        let invaded = format!("{stage} (''Invasion Stage'')");
        next = merge_nav(next, invaded);
    }

    if let Some(continue_data) = stage.continue_data.as_ref() {
        next = merge_nav(next, get_continuation_stages(continue_data));
    }

    (prev, next)
}

/// Get the `prev stage` and `next stage` infobox parameters.
pub fn stage_nav(stage: &Stage, data: &StageWikiDataContainer) -> Vec<TemplateParameter> {
    if [T::Dojo, T::RankingDojo].contains(&stage.id.variant()) {
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
    use crate::interface::scripts::stage_info::stage_info::get_stage_wiki_data;

    #[test]
    fn test_single_stage() {
        let earthshaker = Stage::from_id_current(StageID::from_components(T::SoL, 0, 0)).unwrap();
        let data = get_stage_wiki_data(&earthshaker.id);
        assert_eq!(star(&earthshaker), TemplateParameter::new("star", "4"));
        assert_eq!(
            chapter(&earthshaker, &data),
            vec![TemplateParameter::new(
                "sub-chapter",
                "[[The Legend Begins]]"
            )]
        );
        assert_eq!(max_clears(&earthshaker), None);
        assert_eq!(
            difficulty(&earthshaker),
            Some(TemplateParameter::new("difficulty", "★1"))
        );
        assert_eq!(
            stage_nav(&earthshaker, &data),
            vec![
                TemplateParameter::new("prev stage", "N/A"),
                TemplateParameter::new("next stage", "[[Return of Terror]]")
            ]
        );
    }

    #[test]
    fn test_dojo() {
        let wanderer = Stage::from_id_current(StageID::from_components(T::Dojo, 0, 0)).unwrap();
        let data = get_stage_wiki_data(&wanderer.id);
        assert_eq!(
            chapter(&wanderer, &data),
            vec![TemplateParameter::new(
                "dojo-chapter",
                "[[Catclaw Dojo|Hall of Initiates]]"
            )]
        );

        let crimson_trial_arena =
            Stage::from_id_current(StageID::from_components(T::RankingDojo, 0, 0)).unwrap();
        let data = get_stage_wiki_data(&crimson_trial_arena.id);
        assert_eq!(
            chapter(&crimson_trial_arena, &data),
            vec![TemplateParameter::new("dojo-chapter", "[[Arena of Honor]]")]
        );

        let rankup1 =
            Stage::from_id_current(StageID::from_components(T::Championships, 0, 0)).unwrap();
        let data = get_stage_wiki_data(&rankup1.id);
        assert_eq!(
            chapter(&rankup1, &data),
            vec![TemplateParameter::new(
                "dojo-chapter",
                "[[Catclaw Championships#Catclaw Championships Rank 1|Catclaw Championships Rank 1]]"
            )]
        );
    }

    #[test]
    fn test_old_map() {
        let athletic_meet =
            Stage::from_id_current(StageID::from_components(T::Event, 8, 0)).unwrap();
        let data = get_stage_wiki_data(&athletic_meet.id);
        assert_eq!(
            data.stage_map.name,
            "[[Autumn = Sports Day! (Monthly Event)#Autumn Sports Day|Autumn Sports Day]] (Removed)"
        );
        assert_eq!(
            chapter(&athletic_meet, &data),
            vec![TemplateParameter::new(
                "event-chapter",
                "[[Autumn = Sports Day! (Monthly Event)#Autumn Sports Day|Autumn Sports Day]]"
            )]
        );
    }

    #[test]
    fn test_max_clears() {
        let steel_visage =
            Stage::from_id_current(StageID::from_components(T::Event, 78, 0)).unwrap();
        assert_eq!(
            max_clears(&steel_visage),
            Some(TemplateParameter::new("max clears", "1"))
        );
    }

    #[test]
    fn test_conditional_continue_single() {
        let spectrum_of_truth =
            Stage::from_id_current(StageID::from_components(T::Event, 222, 0)).unwrap();
        let data = get_stage_wiki_data(&spectrum_of_truth.id);
        assert_eq!(
            stage_nav(&spectrum_of_truth, &data),
            vec![
                TemplateParameter::new("prev stage", "N/A"),
                TemplateParameter::new(
                    "next stage",
                    "[[Miracle Iris (Deadly)]] (''Continuation Stage'', 40%)"
                ),
            ]
        );
    }

    #[test]
    fn test_conditional_continue_multiple() {
        let green_envy_3 =
            Stage::from_id_current(StageID::from_components(T::Event, 97, 2)).unwrap();
        let data = get_stage_wiki_data(&green_envy_3.id);
        assert_eq!(
            stage_nav(&green_envy_3, &data),
            vec![
                TemplateParameter::new("prev stage", "[[Green Envy (Expert)]]"),
                TemplateParameter::new(
                    "next stage",
                    "[[Catfruit Jubilee]] (''Continuation Stage'', 5%)<br>\n\
                    [[Catfruit Jubilee]] (''Continuation Stage'', 5%)<br>\n\
                    [[Catfruit Jubilee]] (''Continuation Stage'', 5%)"
                )
            ]
        );
    }

    #[test]
    fn test_continue_stage_nav() {
        let proving_grounds =
            Stage::from_id_current(StageID::from_components(T::Event, 250, 2)).unwrap();
        let data = get_stage_wiki_data(&proving_grounds.id);
        assert_eq!(
            max_clears(&proving_grounds),
            Some(TemplateParameter::new("max clears", "1"))
        );
        assert_eq!(
            stage_nav(&proving_grounds, &data),
            vec![
                TemplateParameter::new("prev stage", "[[First Round (Expert)]]"),
                TemplateParameter::new(
                    "next stage",
                    "[[2nd Round: Dawn (Deadly)]] (''Continuation Stage'')<br>\n\
                    [[2nd Round: Dusk (Deadly)]] (''Continuation Stage'')"
                )
            ]
        );
    }

    #[test]
    fn test_ex_invasion() {
        let sweet_potato_province =
            Stage::from_id_current(StageID::from_components(T::Event, 385, 0)).unwrap();
        let data = get_stage_wiki_data(&sweet_potato_province.id);
        assert_eq!(
            stage_nav(&sweet_potato_province, &data),
            vec![
                TemplateParameter::new("prev stage", "N/A"),
                TemplateParameter::new(
                    "next stage",
                    "[[Ahosan Domain]]<br>\n\
                    [[Doge's Rebellion (Satsuma Imo Domain)|Doge's Rebellion]] (''Invasion Stage'')"
                )
            ]
        );
    }
}
