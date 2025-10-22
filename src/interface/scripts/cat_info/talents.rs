//! Deals with talents section.

use crate::{
    game_data::cat::{
        parsed::{
            cat::Cat,
            stats::form::LATEST_ENEMY_TYPE,
            talents::{SingleTalent, TalentTargets},
        },
        raw::talents_cost::TalentsCostContainer,
    },
    interface::{
        config::Config, error_handler::InfallibleWrite,
        scripts::cat_info::stats::abilities::pure_abilities::get_multiple_hit_abilities,
    },
    wiki_data::talent_names::TALENT_DATA,
    wikitext::{
        number_utils::{get_formatted_float, time_repr},
        section::Section,
    },
};
use num_format::{Locale, ToFormattedString, WriteFormatted};
use std::fmt::Write;

// Is it better to use the ability id and check cat talents, or is it better to
// use the description number?
fn talent_from_text_id(
    talent: &SingleTalent,
    new_targets_with_space: &str,
    multab: &str,
) -> Option<String> {
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

    /// Returns `Err` if step cannot evenly be split up.
    fn calculate_step_inner(talent: &SingleTalent, min: u16, max: u16) -> Result<u16, (u16, u16)> {
        let s = max - min;
        let t = u16::from(talent.max_level) - 1;
        if s % t != 0 {
            return Err((s, t));
        }
        Ok(s / t)
    }

    /// Calculate the step between each level of the talent. Must give out
    /// integers.
    fn calculate_step_exact(talent: &SingleTalent, min: u16, max: u16) -> u16 {
        calculate_step_inner(talent, min, max).expect("step is not evenly divisible")
    }

    fn fmt_inexact_step(s: u16, t: u16) -> String {
        "~".to_string() + &get_formatted_float(f64::from(s) / f64::from(t), 2)
    }

    /// Calculate the step between each level of the talent.
    fn calculate_step(talent: &SingleTalent, min: u16, max: u16) -> String {
        match calculate_step_inner(talent, min, max) {
            Ok(step) => step.to_string(),
            Err((s, t)) => fmt_inexact_step(s, t),
        }
    }

    /// Get representation of time.
    fn fmt_time(param: u16) -> String {
        let (f, s) = time_repr(param.into());
        format!("{f}f <sup>{s}s</sup>")
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
        (91, 66), // sage slayer
    ];

    match talent.skill_description_id {
        1 | 70 | 71 => {
            // unlock weaken, level up for higher duration
            // 70 = relic
            // 71 = alien
            assert_eq!(c_abil, 1);
            assert_eq!(p_len, 3);

            // chance,duration,percent
            let chance = min_is_max!(0);
            let (min, max) = talent.params[1];
            let percent = min_is_max!(2);

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);

            let step = calculate_step_exact(talent, min, max);
            let step_time = fmt_time(step);

            let msg = format!(
                "Adds a {chance}% chance to weaken{new_targets_with_space} enemies to {percent}% for {min_time}{multab}, improves by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        2 | 76 => {
            // freeze, level up for higher duration
            // 76 = metal
            assert_eq!(c_abil, 2);
            assert_eq!(p_len, 2);

            // chance,duration
            let chance = min_is_max!(0);
            let (min, max) = talent.params[1];

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);

            let step = calculate_step_exact(talent, min, max);
            let step_time = fmt_time(step);

            let msg = format!(
                "Adds a {chance}% chance to freeze{new_targets_with_space} enemies for {min_time}{multab}, improves by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        3 | 69 | 72 => {
            // slow, level up for higher duration
            // 69 = relic
            // 72 = metal
            assert_eq!(c_abil, 3);
            assert_eq!(p_len, 2);

            // chance,duration
            let chance = min_is_max!(0);
            let (min, max) = talent.params[1];

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);

            let step = calculate_step_exact(talent, min, max);
            let step_time = fmt_time(step);

            let msg = format!(
                "Adds a {chance}% chance to slow{new_targets_with_space} enemies for {min_time}{multab}, improves by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        8 | 73 | 75 => {
            // knockback, level up for higher chance
            // 73 = zombie
            // 75 = alien
            assert_eq!(c_abil, 8);
            assert_eq!(p_len, 1);

            // chance,duration
            let (min, max) = talent.params[0];

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to knockback{new_targets_with_space} enemies{multab}, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        10 => {
            // strengthen, level up for higher damage
            assert_eq!(c_abil, 10);
            assert_eq!(p_len, 2);

            // hp,damage
            let hp = 100 - min_is_max!(0);
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
                "Adds a {min}% chance to perform a critical hit{multab}, improves by {step}% per level up to {max}%"
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
                "Adds a {min}% chance to break [[Barrier]]s{multab}, improves by {step}% per level up to {max}%"
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
                "Adds a {min}% chance to create a level {level} wave attack{multab}, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        18 | 19 | 20 | 21 | 22 | 24 | 26 | 64 | 66 => {
            // resist {effect}
            let (abil_id, effect) = match talent.skill_description_id {
                18 => (18, "weaken duration"),
                19 => (19, "freeze duration"),
                20 => (20, "slow duration"),
                21 => (21, "knockback push"),
                22 => (22, "wave damage"),
                // 24 => (24, ""),
                24 => unimplemented!(),
                26 => (30, "curse duration"),
                64 => (52, "toxic damage"),
                66 => (54, "surge damage"),
                _ => unreachable!(),
            };

            assert_eq!(c_abil, abil_id);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step_exact(talent, min, max);

            if step == min {
                Some(format!(
                    "Reduces {effect} by {step}% per level up to {max}%"
                ))
            } else {
                Some(format!(
                    "Reduces {effect} by {min}%, improves by {step}% per level up to {max}%"
                ))
            }
        }
        27 => {
            // defense buff
            assert_eq!(c_abil, 32);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);
            // might have to end up formatting this properly

            Some(format!("Upgrades health by {step}% per level up to {max}%"))
        }
        28 => {
            // attack buff
            assert_eq!(c_abil, 31);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step_exact(talent, min, max);
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
            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            Some(format!(
                "Upgrades movement speed by {step} per level up to {max}"
            ))
        }
        30 => {
            // knockback buff
            assert_eq!(c_abil, 28);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            Some(format!(
                "Upgrades knockback push by {step}(?) per level up to {max}(?)"
            ))
        }
        31 => {
            // cost decrease
            assert_eq!(c_abil, 25);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step_exact(talent, min, max);
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
            let step = calculate_step_exact(talent, min, max);

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);
            let step_time = fmt_time(step);

            Some(format!(
                "Reduces recharge time by {min_time}, improves by {step_time} per level up to {max_time}"
            ))
        }
        42 => {
            // upgrade weaken, level up for higher duration
            assert_eq!(c_abil, 1);
            assert_eq!(p_len, 1);

            // duration
            let (min, max) = talent.params[0];

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);

            let step = calculate_step_exact(talent, min, max);
            let step_time = fmt_time(step);

            let msg = format!(
                "Increases weaken duration by {min_time}, improves by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        43 => {
            // upgrade freeze, level up for higher duration
            assert_eq!(c_abil, 2);
            assert_eq!(p_len, 1);

            // duration
            let (min, max) = talent.params[0];

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);

            let step = calculate_step_exact(talent, min, max);
            let step_time = fmt_time(step);

            let msg = format!(
                "Increases freeze duration by {min_time}, improves by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        44 => {
            // upgrade slow, level up for higher duration
            assert_eq!(c_abil, 3);
            assert_eq!(p_len, 1);

            // duration
            let (min, max) = talent.params[0];

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);

            let step = calculate_step_exact(talent, min, max);
            let step_time = fmt_time(step);

            let msg = format!(
                "Increases slow duration by {min_time}, improves by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        45 => {
            // improved knockback
            assert_eq!(c_abil, 8);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            match calculate_step_inner(talent, min, max) {
                Ok(step) => {
                    assert_eq!(step, min);
                    Some(format!(
                        "Increases knockback chance by {step}% per level up to {max}%"
                    ))
                }
                Err((s, t)) => {
                    if s == min {
                        todo!()
                    } else {
                        let step = fmt_inexact_step(s, t);
                        Some(format!(
                            "Increases knockback chance by {min}%, improves by {step}% per level up to {max}%"
                        ))
                    }
                }
            }
        }
        46 => {
            // upgrade strengthen
            assert_eq!(c_abil, 10);
            assert_eq!(p_len, 1);

            // damage
            let (min, max) = talent.params[0];

            let step = calculate_step_exact(talent, min, max);

            let msg = if step == min {
                format!("Upgrades strengthen attack power by {step}% per level up to {max}%")
            } else {
                format!(
                    "Upgrades strengthen attack power by {min}%, improves by {step}% per level up to {max}%"
                )
            };
            Some(msg)
        }
        47 => {
            // upgrade survive
            assert_eq!(c_abil, 11);
            assert_eq!(p_len, 1);

            // chance
            let (min, max) = talent.params[0];

            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            let msg = format!(
                "Upgrades chance to survive lethal strikes by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        48 => {
            // upgrade survive
            assert_eq!(c_abil, 13);
            assert_eq!(p_len, 1);

            // chance
            let (min, max) = talent.params[0];

            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            let msg = format!(
                "Upgrades chance to perform a critical hit by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        49 => {
            // upgrade barrier breaker
            assert_eq!(c_abil, 13);
            assert_eq!(p_len, 1);

            // chance
            let (min, max) = talent.params[0];

            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            let msg =
                format!("Upgrades chance to break [[Barrier]]s by {step}% per level up to {max}%");
            Some(msg)
        }
        50 => {
            // upgrade wave, level up for higher chance
            assert_eq!(c_abil, 17);
            assert_eq!(p_len, 2);

            // chance,level
            let (min, max) = talent.params[0];
            let _level = min_is_max!(1);

            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            let msg =
                format!("Upgrade chance to create wave attacks by {step}% per level up to {max}%");
            Some(msg)
        }
        51 => {
            // upgrade warp
            assert_eq!(c_abil, 9);
            assert_eq!(p_len, 2);

            unimplemented!()
            // // chance,level
            // let (min, max) = talent.params[0];
            // let _level = min_is_max!(1);

            // let step = calculate_step(talent, min, max);
            // assert_eq!(step, min);

            // let msg = format!(
            //     "Upgrade warp distance"
            // );
            // Some(msg)
        }
        52 => {
            // critical, no level-ups (e.g. Cameraman)
            assert_eq!(c_abil, 13);
            assert_eq!(p_len, 1);
            assert!(talent.max_level <= 1);

            // chance
            let chance = min_is_max!(1);

            let msg = format!("Adds a {chance}% chance to perform a critical hit");
            Some(msg)
        }
        59 => {
            // savage blow, increase chance
            assert_eq!(c_abil, 50);
            assert_eq!(p_len, 2);

            // chance,damage
            let (min, max) = talent.params[0];
            let damage = min_is_max!(1);

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to land a savage blow for +{damage}%{multab}, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        60 | 87 => {
            // dodge, increase duration
            // 87 = traitless
            assert_eq!(c_abil, 51);
            assert_eq!(p_len, 2);

            // chance, duration
            let chance = min_is_max!(0);
            let (min, max) = talent.params[1];

            let step = calculate_step_exact(talent, min, max);

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);
            let step_time = fmt_time(step);

            let msg = format!(
                "Adds a {chance}% chance to dodge{new_targets_with_space} enemy attacks for {min_time}, improves by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        61 => {
            // upgrade savage blow, increase chance
            assert_eq!(c_abil, 50);
            assert_eq!(p_len, 2);

            // chance,damage
            let (min, max) = talent.params[0];
            let _damage = min_is_max!(1);

            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            let msg =
                format!("Upgrades chance to land a savage blow by {step}% per level up to {max}%");
            Some(msg)
        }
        62 => {
            // upgrade dodge, increase duration
            assert_eq!(c_abil, 51);
            assert_eq!(p_len, 2);

            // chance, duration
            let _chance = min_is_max!(0);
            let (min, max) = talent.params[1];

            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            let step_time = fmt_time(min);
            let max_time = fmt_time(max);

            let msg = format!(
                "Upgrades chance to dodge attacks by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        63 => {
            // upgrade slow, level up for higher chance
            assert_eq!(c_abil, 3);
            assert_eq!(p_len, 2);

            // chance,duration
            let (min, max) = talent.params[0];
            let _duration = min_is_max!(1);

            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            let msg = format!("Increases slow chance by {step}% per level up to {max}%");
            Some(msg)
        }
        68 => {
            // Unlock surge, level up for higher chance
            assert_eq!(c_abil, 56);
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
                "Adds a {min}% chance to create a level {level} surge attack between {rng1}~{rng2} range{multab}, improves by {step}% per level up to {max}%",
                rng1 = rng_min.to_formatted_string(&Locale::en),
                rng2 = rng_max.to_formatted_string(&Locale::en)
            );
            Some(msg)
        }
        74 => {
            // upgrade freeze, level up for higher chance
            assert_eq!(c_abil, 2);
            assert_eq!(p_len, 1);

            // chance
            let (min, max) = talent.params[0];
            let _duration = min_is_max!(1);

            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            let msg = format!("Increases freeze chance by {step}% per level up to {max}%");
            Some(msg)
        }
        78 => {
            // shield piercer
            assert_eq!(c_abil, 58);
            assert_eq!(p_len, 1);

            // chance
            let (min, max) = talent.params[0];

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to instantly pierce shields{multab}, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        80 => {
            // curse, level up duration
            assert_eq!(c_abil, 60);
            assert_eq!(p_len, 2);

            // chance,duration
            let chance = min_is_max!(0);
            let (min, max) = talent.params[1];

            let step = calculate_step_exact(talent, min, max);

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);
            let step_time = fmt_time(step);

            let msg = format!(
                "Adds a {chance}% chance to curse enemies for {min_time}{multab}, improves by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        81 => {
            // upgrade dodge, increase chance
            assert_eq!(c_abil, 51);
            assert_eq!(p_len, 2);

            // chance, duration
            let (min, max) = talent.params[0];
            let _duration = min_is_max!(1);

            let step = calculate_step_exact(talent, min, max);
            assert_eq!(step, min);

            let msg =
                format!("Upgrade chance to dodge enemy attacks by {step}% per level up to {max}%");
            Some(msg)
        }
        82 => {
            // attack frequency up
            assert_eq!(c_abil, 61);
            assert_eq!(p_len, 1);

            let (min, max) = talent.params[0];
            let step = calculate_step_exact(talent, min, max);

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);
            let step_time = fmt_time(step);

            Some(format!(
                "Reduces attack cooldown by {min_time}, improves by {step_time} per level up to {max_time}"
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
                "Adds a {min}% chance to create a level {level} mini-wave{multab}, improves by {step}% per level up to {max}%"
            );
            Some(msg)
        }
        84 => {
            // "Immune to zombies"
            unimplemented!()
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
                "Adds a {min}% chance to create a level {level} mini-surge between {rng1}~{rng2} range{multab}, improves by {step}% per level up to {max}%",
                rng1 = rng_min.to_formatted_string(&Locale::en),
                rng2 = rng_max.to_formatted_string(&Locale::en)
            );
            Some(msg)
        }
        88 | 90 | 95 => {
            // Unlock dodge, level up for higher chance
            // 88 = Nekoluga
            // 90 = all
            // 95 = metal
            assert_eq!(c_abil, 51);
            assert_eq!(p_len, 2);

            let (min, max) = talent.params[0];
            let duration = min_is_max!(1);

            let duration = fmt_time(duration);
            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to dodge{new_targets_with_space} enemy attacks for {duration}, improves by {step}% per level up to {max}%"
            );

            Some(msg)
        }
        92 => {
            // none_map but with a target
            // 92 = strong against relics
            assert_eq!(c_abil, 5);
            assert_eq!(p_len, 0);
            assert!(talent.max_level <= 1);

            assert!(!new_targets_with_space.is_empty());
            Some(new_targets_with_space.to_string())
        }
        93 => {
            // upgrade curse, level up duration
            assert_eq!(c_abil, 60);
            assert_eq!(p_len, 2);

            // chance,duration
            let _chance = min_is_max!(0);
            let (min, max) = talent.params[1];

            let step = calculate_step_exact(talent, min, max);

            let min_time = fmt_time(min);
            let max_time = fmt_time(max);
            let step_time = fmt_time(step);

            let msg = format!(
                "Upgrades chance to curse enemies by {min_time}, improves by {step_time} per level up to {max_time}"
            );
            Some(msg)
        }
        94 => {
            // explosion, level up chance
            assert_eq!(c_abil, 67);
            assert_eq!(p_len, 2);

            // chance,spawn_quad
            let (min, max) = talent.params[0];
            let spawn_quad = min_is_max!(1);

            let step = calculate_step(talent, min, max);

            let msg = format!(
                "Adds a {min}% chance to create an explosion at {rng1} range{multab}, improves by {step}% per level up to {max}%",
                rng1 = (spawn_quad / 4).to_formatted_string(&Locale::en),
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

fn get_single_talent(
    talent: &SingleTalent,
    config: &Config,
    targs: &[TalentTargets],
    multab: &str,
) -> String {
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

    if let Some(desc) = talent_from_text_id(talent, new_targets_with_space, multab) {
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

    let tf_multab = cat
        .forms
        .stats
        .get(2)
        .map(|stats| get_multiple_hit_abilities(&stats.attack.hits));

    let uf_multab = cat
        .forms
        .stats
        .get(3)
        .map(|stats| get_multiple_hit_abilities(&stats.attack.hits));

    if tf_multab != uf_multab && uf_multab.is_some() {
        panic!("TF and UF have different `multab`s!")
    }

    let mut normal = vec![];
    for talent in talents.normal {
        normal.push(get_single_talent(
            &talent,
            config,
            &talents.implicit_targets,
            tf_multab.unwrap(),
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
            tf_multab.unwrap(),
        ));
    }

    if ultra.is_empty() {
        return Some(Section::h2(TITLE, buf));
    }

    buf += "\n\n===Ultra Talents===\n";
    buf += &ultra.join("\n");

    Some(Section::h2(TITLE, buf))
}

#[cfg(test)]
mod tests {
    #[test]
    fn approximate_scaling() {
        // dark lazer
        todo!()
    }
}
