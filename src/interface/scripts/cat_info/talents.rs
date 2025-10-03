//! Deals with talents section.

use std::fmt::Write;

use num_format::{Locale, ToFormattedString};

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
    wikitext::{number_utils::time_repr, section::Section},
};

// Is it better to use the ability id and check cat talents, or is it better to
// use the description number?
fn talent_from_text_id(talent: &SingleTalent, new_targets_with_space: &str) -> Option<String> {
    log::debug!("{talent:?}/{new_targets_with_space}");

    let c_abil = usize::from(talent.ability_id);
    // check ability id
    let p_len = talent.params.len();

    /// Assert that the minimum parameter is actually the same as the maximum.
    /// Also return the min/max.
    macro_rules! min_is_max {
        ($id:expr) => {{
            assert_eq!(talent.params[$id].0, talent.params[$id].1);
            talent.params[$id].0
        }};
    }

    /// Calculate the step between each level of the talent.
    fn calculate_step(talent: &SingleTalent, min: u16, max: u16) -> u16 {
        let s = max - min;
        let t = u16::from(talent.max_level) - 1;
        assert_eq!(s % t, 0);
        s / t
    }

    match talent.skill_description_id {
        1 => {
            // weaken, ???
            assert_eq!(c_abil, 1);
            assert_eq!(p_len, 3);

            todo!()
        }
        28 => {
            // attack buff
            assert_eq!(c_abil, 31);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step(talent, min, max);
            assert_eq!(step, min);
            // might have to end up formatting this properly

            Some(format!(
                "Upgrades attack power by {step}% per level up to {max}%"
            ))
        }
        89 => {
            // Unlock mini-surge, level up for higher chance
            assert_eq!(c_abil, 65);
            assert_eq!(p_len, 4);

            // chance,level,spawn_quad,range
            let (min, max) = talent.params[0];
            let level = min_is_max!(1);
            let spawn_quad = min_is_max!(2);
            let range_quad = min_is_max!(3);

            let rng_min = spawn_quad / 4;
            let rng_max = rng_min + range_quad / 4;

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to create a level {level} mini-surge between {rng1}~{rng2} range, improves by {step}% per level up to {max}%",
                rng1 = rng_min.to_formatted_string(&Locale::en),
                rng2 = rng_max.to_formatted_string(&Locale::en)
            );
            Some(msg)
        }
        90 => {
            // Unlock dodge, level up for higher chance
            assert_eq!(c_abil, 51);
            assert_eq!(p_len, 2);

            let (min, max) = talent.params[0];
            let duration = min_is_max!(1);

            let (min_dur, max_dur) = time_repr(duration.into());
            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to dodge{new_targets_with_space} enemy attacks for {min_dur}f<sup>{max_dur}s</sup>, improves by {step}% per level up to {max}%"
            );

            Some(msg)
        }
        id => {
            log::warn!("Unknown skill description id: {id}");
            Some(String::from("???"))
        }
    }
}

fn get_single_talent(talent: &SingleTalent, config: &Config, targs: &[TalentTargets]) -> String {
    let mut buf = format!(
        "*'''{}'''",
        TALENT_DATA.get_talent_name(talent.ability_id.into())
    );

    let new_targets_with_space = if targs.is_empty() {
        ""
    } else if targs.len() == 1 {
        match &targs[0] {
            TalentTargets::Metal => " [[:Category:Metal Enemies|Metal]]",
            TalentTargets::Alien => " [[:Category:Alien Enemies|Alien]]",
            TalentTargets::Zombie => " [[:Category:Zombie Enemies|Zombie]]",
            TalentTargets::Relic => " [[:Category:Relic Enemies|Relic]]",
            t => panic!("Found {t:?}, not sure what this type is"),
        }
    } else if targs.len() == LATEST_ENEMY_TYPE as usize + 1 {
        " all"
    } else {
        unimplemented!(
            "Found nonstandard enemy list type: length = {}",
            targs.len()
        )
    };

    if let Some(desc) = talent_from_text_id(talent, new_targets_with_space) {
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
        write!(buf, " (Cost: {} NP)", costs[0]).infallible_write();
    } else {
        write!(buf, " (Total Cost: {} NP)", costs.iter().sum::<u16>()).infallible_write();
    }

    buf
}

/// Get the talents section.
pub fn talents_section(cat: &Cat, config: &Config) -> Option<Section> {
    const TITLE: &str = "Talents";
    let talents = cat.get_talents(config.version.current_version())?;

    let mut normal = vec![];
    for talent in talents.normal {
        normal.push(get_single_talent(
            &talent,
            config,
            &talents.implicit_targets,
        ));
    }

    if normal.is_empty() {
        return None;
    }

    let mut buf = normal.join("\n");

    let mut ultra = vec![];
    for talent in talents.ultra {
        ultra.push(get_single_talent(
            &talent,
            config,
            &talents.implicit_targets,
        ));
    }

    if ultra.is_empty() {
        return Some(Section::h2(TITLE, buf));
    }

    buf += "\n\n===Ultra Talents===\n";
    buf += &ultra.join("\n");

    Some(Section::h2(TITLE, buf))
}
