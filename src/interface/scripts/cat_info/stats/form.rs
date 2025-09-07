//! Data about a form to put in stats table.

use super::abilities::{misc_abilities::get_range_ability, pure_abilities::get_pure_abilities};
use crate::{
    game_data::cat::parsed::{
        anim::CatFormAnimData,
        cat::Cat,
        stats::form::{AreaOfEffect, CatFormStats},
    },
    interface::{
        error_handler::InfallibleWrite,
        scripts::cat_info::stats::abilities::{
            misc_abilities::get_multihit_ability, util::get_ability_single,
        },
    },
    wikitext::number_utils::{
        get_formatted_float, plural, plural_f, seconds_repr, time_repr, time_repr_i32,
    },
};
use num_format::{Locale, ToFormattedString};
use std::{cmp::max, fmt::Write};

/// Container for cat form data. Includes form's base stats and stats level.
///
/// Struct is used so that this can work in a parameter-name-agnostic fashion
/// (i.e. it works fine whether parameter name is "Normal Health" or
/// "val-Normal-Health" or whatever the old template's name for it was). All
/// info is formatted strings.
pub struct FormWithBaseStats {
    /// Level cat's stats are shown at.
    pub stats_level: Option<String>,
    /// Cat's base HP.
    pub base_hp: String,
    /// Cat's base attack/dps.
    pub base_atk: String,
    /// Rest of form data.
    pub other: Form,
}

/// Parameter-name-agnostic container for formatted Cat Stats info.
pub struct Form {
    /// Standing range.
    pub range: String,
    /// Attack cycle in frames and seconds.
    pub attack_cycle: String,
    /// Unit speed.
    pub speed: String,
    /// Knockbacks.
    pub knockback: String,
    /// Foreswing/backswing.
    pub animation: String,
    /// Recharge time (max ~ min seconds).
    pub recharge: String,
    /// HP at max level.
    pub hp_max: String,
    /// AP at max level.
    pub atk_max: String,
    /// Single/Area.
    pub attack_type: &'static str,
    /// List of cat abilities.
    pub abilities: String,
}

/// Write natural and plus level to a buffer. If `plus` is 0, only writes `nat`.
pub fn write_level_and_plus(buf: &mut String, nat: u8, plus: u8) {
    write!(buf, "{nat}").infallible_write();
    if plus > 0 {
        write!(buf, "+{plus}").infallible_write();
    }
}

/// Get all of the [`FormWithBaseStats`] data for the cat form.
pub fn get_form(
    cat: &Cat,
    stats: &CatFormStats,
    anims: &CatFormAnimData,
    form_no: u8,
) -> FormWithBaseStats {
    let max_levels = &cat.unitbuy.max_levels;
    log::debug!("{cat:?}");
    // don't have all information yet so debug in case info is needed

    let nat_and_plus = (max_levels.max_nat, max_levels.max_plus);
    let levels_used = match nat_and_plus {
        (1, 30) => (1, 30),
        (1, 29..) => (1, 29),
        (0..=29, _) => (max_levels.max_nat, max_levels.max_plus),
        _ => (30, 0),
    };
    let level = if form_no == 4 {
        60
    } else {
        levels_used.0 + levels_used.1
    };

    let foreswing = stats.attack.hits.foreswing();
    let attack_length = stats.attack.hits.attack_length();
    let anim_length = anims.attack.length();

    let can_attack = anim_length > attack_length;
    // if animation is shorter than foreswing then unit cannot attack
    // unclear if should be > or >=
    if !can_attack {
        log::info!(
            "Attack animation length mismatch: {anim_length} <= {attack_length}. Unit cannot attack.",
        );
    }

    let frequency_opt = if can_attack && !stats.attack.kamikaze {
        Some(max(anim_length, attack_length + stats.attack.cooldown))
    } else {
        None
    };

    let stats_level = match levels_used {
        (30, 0) => None,
        (nat, plus) => {
            let mut buf = String::new();
            write_level_and_plus(&mut buf, nat, plus);
            Some(buf)
        }
    };

    let base_hp = format!("{hp} HP", hp = stats.hp.to_formatted_string(&Locale::en));
    let base_atk = {
        let dmg = stats.attack.hits.total_damage();
        let mut buf = format!("{ap} damage", ap = dmg.to_formatted_string(&Locale::en),);

        if let Some(frequency) = frequency_opt {
            let dps_init = f64::from(dmg) / f64::from(frequency) * 30.0;
            write!(
                buf,
                "<br>({dps} DPS)",
                dps = get_formatted_float(dps_init, 2)
            )
            .infallible_write();
        }

        buf
    };

    let range = stats.attack.standing_range.to_formatted_string(&Locale::en);
    let attack_cycle = if let Some(frequency) = frequency_opt {
        let (freq_f, freq_s) = time_repr(u32::from(frequency));
        format!(
            "{freq_f}f <sub>{freq_s} {seconds}</sub>",
            seconds = plural_f(frequency.into(), "second", "seconds")
        )
    } else if can_attack {
        "-".to_string()
    } else {
        "Cannot attack".to_string()
    };

    let speed = stats.speed.to_formatted_string(&Locale::en);
    let knockback = format!(
        "{kb} {times}",
        kb = stats.kb.to_formatted_string(&Locale::en),
        times = plural(stats.kb, "time", "times")
    );
    let animation = {
        let (fore_f, fore_s) = time_repr(u32::from(foreswing));
        let backswing = anim_length as i32 - attack_length as i32;
        let (back_f, back_s) = time_repr_i32(backswing);
        format!("{fore_f}f <sup>{fore_s}s</sup><br>({back_f}f <sup>{back_s}s</sup> backswing)")
    };

    let recharge = {
        let max_spawn = stats.respawn_half * 2;
        let min_spawn = {
            const MAX_LEVEL_REDUCE_F: u16 = 264;
            // 8.8 * 30
            const MIN_SPAWN_AMT: u16 = 60;
            // 2 seconds
            max(max_spawn, MAX_LEVEL_REDUCE_F + MIN_SPAWN_AMT) - MAX_LEVEL_REDUCE_F
            // because this uses unsigned integers, the intuitive `max(2s,
            // base_spawn - 8.8s)` could loop around to `u32::MAX`, so `max`
            // needs to be applied beforehand
        };
        let max_s = seconds_repr(max_spawn.into());
        let min_s = seconds_repr(min_spawn.into());
        format!("{max_s} ~ {min_s} seconds")
        // no need for plural as min is 2 seconds
    };

    let hp_max = {
        let hp_max = cat.unitlevel.get_stat_at_level(stats.hp, level);
        format!("{hp} HP", hp = hp_max.to_formatted_string(&Locale::en))
    };
    let atk_max = {
        let ap_max = stats
            .attack
            .hits
            .total_damage_at_level(&cat.unitlevel, level);
        let mut buf = format!("{ap} damage", ap = ap_max.to_formatted_string(&Locale::en),);

        if let Some(frequency) = frequency_opt {
            let dps_max = f64::from(ap_max) / f64::from(frequency) * 30.0;
            write!(
                buf,
                "<br>({dps} DPS)",
                dps = get_formatted_float(dps_max, 2)
            )
            .infallible_write();
        }

        buf
    };

    let attack_type = match stats.attack.aoe {
        AreaOfEffect::SingleAttack => "Single",
        AreaOfEffect::AreaAttack => "Area",
    };
    let abilities = {
        let mut abilities = vec![];

        if stats.attack.kamikaze {
            abilities.push(format!(
                "{{{{AbilityIcon|Kamikaze}}}} {kamikaze} (Attacks once, then disappears from the battlefield)",
                kamikaze = get_ability_single("Kamikaze")
            ))
        }
        abilities.extend(get_multihit_ability(
            &stats.attack.hits,
            &cat.unitlevel,
            level,
        ));
        abilities.extend(get_range_ability(&stats.attack.hits));
        abilities.extend(get_pure_abilities(
            &stats.attack.hits,
            &stats.abilities,
            &stats.targets,
        ));

        if abilities.is_empty() {
            "-".to_string()
        } else {
            abilities.join("<br>\n")
        }
    };

    FormWithBaseStats {
        stats_level,
        base_hp,
        base_atk,
        other: Form {
            range,
            attack_cycle,
            speed,
            knockback,
            animation,
            recharge,
            hp_max,
            atk_max,
            attack_type,
            abilities,
        },
    }
}
