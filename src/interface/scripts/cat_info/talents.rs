//! Deals with talents section.

use std::fmt::Write;

use num_format::{Locale, ToFormattedString, WriteFormatted};

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

    // pair of (`skill_description_id`, `ability_id`)
    let none_map = [
        (4, 4),   // attacks only
        (5, 5),   // strong vs
        (6, 6),   // tough vs
        (7, 7),   // massive damage
        (9, 9),   // warp
        (12, 12), // base destroyer
        (14, 14), // zombie killer
        (16, 16), // money up
        (23, 23), // wave immunity
        (25, 29), // curse immunity
        (33, 33), // target red
        (34, 34), // target floating
        (35, 35), // target black
        (36, 36), // target metal
        (37, 37), // target angel
        (38, 38), // target alien
        (39, 39), // target zombie
        (40, 40), // target relic
        (41, 41), // target traitless
        (53, 44), // immune to weaken
        (54, 45), // immune to freeze
        (55, 46), // immune to slow
        (56, 47), // immune to knockback
        (57, 48), // immune to waves (again)
        (58, 49), // warp blocker
        (65, 53), // immune to toxic
        (67, 55), // immune to surge
        (77, 57), // target aku
        (79, 59), // soulstrike
        (85, 63), // colossus slayer
        (86, 64), // behemoth slayer
        (91, 66), // behemoth slayer
        (92, 5),  // strong vs (relic)
    ];

    match talent.skill_description_id {
        1 => {
            // unlock weaken, level up for higher duration
            assert_eq!(c_abil, 1);
            assert_eq!(p_len, 3);

            // chance,duration,percent
            let chance = min_is_max!(0);
            let (min, max) = talent.params[1];
            let percent = min_is_max!(2);

            let (min_f, min_s) = time_repr(min.into());
            let (max_f, max_s) = time_repr(max.into());

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {chance}% chance to weaken enemies to {percent}% for {min_f}f<sup>{min_s}s</sup>, improves by {step}f per level up to {max_f}f<sup>{max_s}s</sup>"
            );
            Some(msg)
        }
        2 => {
            // freeze, level up for higher duration
            assert_eq!(c_abil, 2);
            assert_eq!(p_len, 2);

            // chance,duration
            let chance = min_is_max!(0);
            let (min, max) = talent.params[1];

            let (min_f, min_s) = time_repr(min.into());
            let (max_f, max_s) = time_repr(max.into());

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {chance}% chance to freeze enemies for {min_f}f<sup>{min_s}s</sup>, improves by {step}f per level up to {max_f}f<sup>{max_s}s</sup>"
            );
            Some(msg)
        }
        3 => {
            // slow, level up for higher duration
            assert_eq!(c_abil, 8);
            assert_eq!(p_len, 2);

            // chance,duration
            let chance = min_is_max!(0);
            let (min, max) = talent.params[1];

            let (min_f, min_s) = time_repr(min.into());
            let (max_f, max_s) = time_repr(max.into());

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {chance}% chance to slow enemies for {min_f}f<sup>{min_s}s</sup>, improves by {step}f per level up to {max_f}f<sup>{max_s}s</sup>"
            );
            Some(msg)
        }
        8 => {
            // knockback, level up for higher chance
            assert_eq!(c_abil, 8);
            assert_eq!(p_len, 1);

            // chance,duration
            let (min, max) = talent.params[0];

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to knockback enemies, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        10 => {
            // strengthen, level up for higher damage
            assert_eq!(c_abil, 10);
            assert_eq!(p_len, 2);

            // hp,damage
            let hp = min_is_max!(0);
            let (min, max) = talent.params[1];

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds {min}% attack power at {hp}% health, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        11 => {
            // survives, level up for higher chance
            assert_eq!(c_abil, 11);
            assert_eq!(p_len, 1);

            // chance
            let (min, max) = talent.params[0];

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to survive a lethal strike, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        13 => {
            // critical, level up for higher chance
            assert_eq!(c_abil, 13);
            assert_eq!(p_len, 1);

            // chance
            let (min, max) = talent.params[0];

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to perform a critical hit, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        15 => {
            // barrier breaker, level up for higher chance
            assert_eq!(c_abil, 15);
            assert_eq!(p_len, 1);

            // chance
            let (min, max) = talent.params[0];

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to break [[Barrier]]s, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        17 => {
            // Unlock wave, level up for higher chance
            assert_eq!(c_abil, 17);
            assert_eq!(p_len, 2);

            // chance,level
            let (min, max) = talent.params[0];
            let level = min_is_max!(1);

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to create a level {level} wave attack, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        18 | 19 | 20 | 21 | 22 | 24 | 26 => {
            let (abil_id, effect) = match talent.skill_description_id {
                18 => (18, "weaken duration"),
                19 => (19, "freeze duration"),
                20 => (20, "slow duration"),
                21 => (21, "knockback push"),
                22 => (22, "wave damage"),
                // 24 => (24, ""),
                24 => unreachable!(),
                26 => (30, "curse duration"),
                _ => unreachable!(),
            };

            assert_eq!(c_abil, abil_id);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step(talent, min, max);
            assert_eq!(step, min);

            Some(format!(
                "Reduces {effect} by {step}% per level up to {max}%"
            ))
        }
        27 => {
            // defense buff
            assert_eq!(c_abil, 32);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step(talent, min, max);
            assert_eq!(step, min);
            // might have to end up formatting this properly

            Some(format!("Upgrades health by {step}% per level up to {max}%"))
        }
        28 => {
            // attack buff
            assert_eq!(c_abil, 31);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step(talent, min, max);
            assert_eq!(step, min);

            Some(format!(
                "Upgrades attack power by {step}% per level up to {max}%"
            ))
        }
        29 => {
            // speed buff
            assert_eq!(c_abil, 27);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step(talent, min, max);
            assert_eq!(step, min);

            Some(format!(
                "Upgrades movement speed by {step}% per level up to {max}%"
            ))
        }
        30 => {
            // knockback buff
            assert_eq!(c_abil, 28);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step(talent, min, max);
            assert_eq!(step, min);

            Some(format!(
                "Upgrades knockback push by {step}% per level up to {max}%"
            ))
        }
        31 => {
            // cost decrease
            assert_eq!(c_abil, 25);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step(talent, min, max);
            assert_eq!(step, min);

            fn get_all_costs(eoc1: u16) -> String {
                let mut buf = String::new();
                buf.write_formatted(&eoc1, &Locale::en).infallible_write();
                buf += "/";
                buf.write_formatted(&(eoc1 * 3 / 2), &Locale::en)
                    .infallible_write();
                buf += "/";
                buf.write_formatted(&(eoc1 * 2), &Locale::en)
                    .infallible_write();

                buf
            }

            Some(format!(
                "Reduces deploy cost by {step_all}¢ per level up to {max_all}¢",
                step_all = get_all_costs(step),
                max_all = get_all_costs(max)
            ))
        }
        32 => {
            // recover speed down
            assert_eq!(c_abil, 26);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step(talent, min, max);

            let (min_f, min_s) = time_repr(min.into());
            let (max_f, max_s) = time_repr(max.into());

            Some(format!(
                "Reduces recharge time by {min_f}f<sup>{min_s}s</sup>, improves by {step}f per level up to {max_f}f<sup>{max_s}s</sup>"
            ))
        }
        42
        45 => {
            // improved knockback
            assert_eq!(c_abil, 8);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step(talent, min, max);
            assert_eq!(step, min);

            Some(format!(
                "Increases knockback chance by {step}% per level up to {max}%"
            ))
        }
        83 => {
            // Unlock mini-wave, level up for higher chance
            assert_eq!(c_abil, 62);
            assert_eq!(p_len, 2);

            // chance,level
            let (min, max) = talent.params[0];
            let level = min_is_max!(1);

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to create a level {level} mini-wave, improves by {step}% per level up to {max}%"
            );
            Some(msg)
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
            let Some(pair) = none_map.iter().find(|(desc_id, _)| *desc_id == id) else {
                log::warn!("Unknown skill description id: {id}");
                return Some(String::from("???"));
            };

            assert_eq!(c_abil, pair.1);
            assert_eq!(p_len, 0);
            assert!(talent.max_level <= 1);

            None
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
