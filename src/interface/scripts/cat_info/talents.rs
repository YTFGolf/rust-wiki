//! Deals with talents section.

use std::fmt::Write;

use crate::{
    game_data::cat::{
        parsed::{
            cat::Cat,
            stats::form::LATEST_ENEMY_TYPE,
            talents::{SingleTalent, TalentTargets},
        },
        raw::talents_cost::TalentsCostContainer,
    },
    interface::{config::Config, error_handler::InfallibleWrite},
    wiki_data::talent_names::TALENT_DATA,
    wikitext::section::Section,
};

// Is it better to use the ability id and check cat talents, or is it better to
// use the description number?
fn talent_from_text_id(talent: &SingleTalent, new_targets: &str) -> Option<String> {
    log::debug!("{talent:?}/{new_targets}");
    match talent.skill_description_id {
        1 => {
            // weaken
            assert_eq!(usize::from(talent.ability_id), 1_usize);
            assert_eq!(talent.params.len(), 3);

            todo!()
        }
        id => {
            log::warn!("Unknown skill description id: {id}");
            Some(String::from("???"))
        }
    }
}

fn get_single_talent(talent: &SingleTalent, config: &Config, targs: &[TalentTargets]) -> String {
    // for talent in talents.groups.iter() {
    //     println!("{talent:?}");
    //     println!(
    //         "abilityID_X = {}",
    //         TALENT_DATA.get_talent_name(talent.abilityID_X.into())
    //     )
    // }

    let mut buf = format!(
        "'''{}'''",
        TALENT_DATA.get_talent_name(talent.ability_id.into())
    );

    let new_targets = if targs.is_empty() {
        ""
    } else if targs.len() == 1 {
        match &targs[0] {
            TalentTargets::Metal => "[[:Category:Metal Enemies|Metal]]",
            TalentTargets::Alien => "[[:Category:Alien Enemies|Alien]]",
            TalentTargets::Zombie => "[[:Category:Zombie Enemies|Zombie]]",
            TalentTargets::Relic => "[[:Category:Relic Enemies|Relic]]",
            t => panic!("Found {t:?}, not sure what this type is"),
        }
    } else if targs.len() == LATEST_ENEMY_TYPE as usize + 1 {
        "all"
    } else {
        unimplemented!(
            "Found nonstandard enemy list type: length = {}",
            targs.len()
        )
    };

    if let Some(desc) = talent_from_text_id(talent, new_targets) {
        write!(buf, ": {desc}").infallible_write();
    }

    let costs_cont = config
        .version
        .current_version()
        .get_cached_file::<TalentsCostContainer>();
    let costs = &costs_cont
        .from_cost_id(talent.skill_costs_id)
        .unwrap()
        .costs;
    if costs.len() == 1 {
        write!(buf, " (Cost: {})", costs[0]).infallible_write();
    } else {
        write!(buf, " (Total cost: {})", costs.iter().sum::<u16>()).infallible_write();
    }

    buf
}

/// Get the talents section.
pub fn talents_section(cat: &Cat, config: &Config) -> Option<Section> {
    let talents = cat.get_talents(config.version.current_version())?;

    let mut normal = vec![];
    for talent in talents.normal {
        normal.push(get_single_talent(
            &talent,
            config,
            &talents.implicit_targets,
        ));
    }

    if true {
        panic!("{normal:?}");
    }

    None
}
