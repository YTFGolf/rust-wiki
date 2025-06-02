use super::abilities::{misc_abilities::get_range_ability, pure_abilities::get_pure_abilities};
use crate::{
    game_data::cat::parsed::{
        anim::CatFormAnimData,
        cat::Cat,
        stats::form::{AreaOfEffect, CatFormStats},
    },
    interface::{
        error_handler::InfallibleWrite,
        scripts::cat_info::stats::abilities::misc_abilities::get_multihit_ability,
    },
    wikitext::number_utils::{get_formatted_float, plural, plural_f, seconds_repr, time_repr},
};
use num_format::{Locale, ToFormattedString};
use std::{cmp::max, fmt::Write};

pub struct FormWithBaseStats {
    pub base_hp: String,
    pub base_atk: String,
    pub other: Form,
}
pub struct Form {
    pub range: String,
    pub attack_cycle: String,
    pub speed: String,
    pub knockback: String,
    pub animation: String,
    pub recharge: String,
    pub hp_max: String,
    pub atk_max: String,
    pub attack_type: &'static str,
    pub abilities: String,
}

pub fn write_level_and_plus(buf: &mut String, nat: u8, plus: u8) {
    write!(buf, "{}", nat).infallible_write();
    if plus > 0 {
        write!(buf, "+{}", plus).infallible_write();
    }
}

pub fn get_form(cat: &Cat, stats: &CatFormStats, anims: &CatFormAnimData) -> FormWithBaseStats {
    let max_levels = &cat.unitbuy.max_levels;
    let levels_used = match max_levels.max_nat {
        0..=29 => (max_levels.max_nat, max_levels.max_plus),
        _ => (30, 0),
    };
    let level = levels_used.0 + levels_used.1;

    let foreswing = stats.attack.hits.foreswing();
    let attack_length = stats.attack.hits.attack_length();
    let backswing = anims.length() - attack_length;
    let frequency = attack_length + {
        let tba = stats.attack.tba;
        if tba == 0 {
            backswing
        } else {
            max(2 * tba - 1, backswing)
        }
        // necessary to avoid overflow
    };

    let base_hp = format!("{hp} HP", hp = stats.hp.to_formatted_string(&Locale::en));
    let base_atk = {
        let dmg = stats.attack.hits.total_damage();
        let dps = f64::from(dmg) / f64::from(frequency) * 30.0;
        format!(
            "{ap} damage<br>({dps} DPS)",
            ap = dmg.to_formatted_string(&Locale::en),
            dps = get_formatted_float(dps, 2)
        )
    };

    let range = stats.attack.standing_range.to_formatted_string(&Locale::en);
    let attack_cycle = {
        let (freq_f, freq_s) = time_repr(u32::from(frequency));
        format!(
            "{freq_f}f <sub>{freq_s} {seconds}</sub>",
            seconds = plural_f(frequency.into(), "second", "seconds")
        )
    };

    let speed = stats.speed.to_formatted_string(&Locale::en);
    let knockback = format!(
        "{kb} {times}",
        kb = stats.kb,
        times = plural(stats.kb, "time", "times")
    );
    let animation = {
        let (fore_f, fore_s) = time_repr(u32::from(foreswing));
        let (back_f, back_s) = time_repr(u32::from(backswing));
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
        let dps_max = f64::from(ap_max) / f64::from(frequency) * 30.0;
        format!(
            "{ap} damage<br>({dps} DPS)",
            ap = ap_max.to_formatted_string(&Locale::en),
            dps = get_formatted_float(dps_max, 2)
        )
    };

    let attack_type = match stats.attack.aoe {
        AreaOfEffect::SingleAttack => "Single Attack",
        AreaOfEffect::AreaAttack => "Area Attack",
    };
    let abilities = {
        let mut abilities = vec![];
        abilities.extend(get_multihit_ability(stats, &cat.unitlevel, level));
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
