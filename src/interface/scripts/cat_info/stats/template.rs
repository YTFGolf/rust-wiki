use super::abilities::{pure_abilities::get_pure_abilities, range_abilities::get_range_ability};
use crate::{
    game_data::cat::{
        parsed::{
            cat::Cat,
            stats::form::{AreaOfEffect, AttackHit, AttackHits, CatFormStats},
        },
        raw::unitlevel::UnitLevelRaw,
    },
    interface::{config::Config, error_handler::InfallibleWrite},
    wiki_data::cat_data::CAT_DATA,
    wikitext::{
        number_utils::{get_formatted_float, plural, plural_f, seconds_repr, time_repr},
        template::{Template, TemplateParameter},
    },
};
use num_format::{Locale, ToFormattedString, WriteFormatted};
use std::{cmp::max, fmt::Write};

pub fn get_template(cat: Cat) {
    let forms = &cat.forms;
    let stats = &forms.stats[0];
    let anims = &forms.anims[0];

    let mut t = Template::named("Cat Stats");

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

    t.push_params(TemplateParameter::new(
        "Normal Form name",
        &CAT_DATA.get_cat(cat.id).normal,
    ));

    t.push_params(TemplateParameter::new(
        "Hp Normal",
        format!("{hp} HP", hp = stats.hp.to_formatted_string(&Locale::en)),
    ));

    let dmg = stats.attack.hits.total_damage();
    let dps = f64::from(dmg) / f64::from(frequency) * 30.0;
    t.push_params(TemplateParameter::new(
        "Atk Power Normal",
        format!(
            "{ap} damage<br>({dps} DPS)",
            ap = dmg.to_formatted_string(&Locale::en),
            dps = get_formatted_float(dps, 2)
        ),
    ));

    t.push_params(TemplateParameter::new(
        "Atk Range Normal",
        stats.attack.standing_range.to_formatted_string(&Locale::en),
    ));

    let (freq_f, freq_s) = time_repr(u32::from(frequency));
    t.push_params(TemplateParameter::new(
        "Attack Frequency Normal",
        format!(
            "{freq_f}f <sub>{freq_s} {seconds}</sub>",
            seconds = plural_f(frequency.into(), "second", "seconds")
        ),
    ));

    t.push_params(TemplateParameter::new(
        "Movement Speed Normal",
        stats.speed.to_formatted_string(&Locale::en),
    ));

    t.push_params(TemplateParameter::new(
        "Knockback Normal",
        format!(
            "{kb} {times}",
            kb = stats.kb,
            times = plural(stats.kb, "time", "times")
        ),
    ));

    let (fore_f, fore_s) = time_repr(u32::from(foreswing));
    let (back_f, back_s) = time_repr(u32::from(backswing));
    t.push_params(TemplateParameter::new(
        "Attack Animation Normal",
        format!("{fore_f}f <sup>{fore_s}s</sup><br>({back_f}f <sup>{back_s}s</sup> backswing)"),
    ));

    let max_spawn = stats.respawn_half * 2;
    let min_spawn = {
        const MAX_LEVEL_REDUCE_F: u16 = 264;
        // 8.8 * 30
        const MIN_SPAWN_AMT: u16 = 60;
        // 2 seconds
        max(max_spawn, MAX_LEVEL_REDUCE_F + MIN_SPAWN_AMT) - MAX_LEVEL_REDUCE_F
        // because this uses unsigned integers, the intuitive `max(2s,
        // base_spawn - 8.8s)` could loop around to `u32::MAX`, so `max` needs
        // to be applied beforehand
    };
    let max_s = seconds_repr(max_spawn.into());
    let min_s = seconds_repr(min_spawn.into());
    t.push_params(TemplateParameter::new(
        "Recharging Time Normal",
        format!("{max_s} ~ {min_s} seconds"),
    ));
    // no need for plural as min is 2 seconds

    let level = 30;

    let hp_max = cat.unitlevel.get_stat_at_level(stats.hp, level);
    t.push_params(TemplateParameter::new(
        "Hp Normal Lv.MAX",
        format!("{hp} HP", hp = hp_max.to_formatted_string(&Locale::en)),
    ));

    let ap_max = stats
        .attack
        .hits
        .total_damage_at_level(&cat.unitlevel, level);
    let dps_max = f64::from(ap_max) / f64::from(frequency) * 30.0;
    t.push_params(TemplateParameter::new(
        "Atk Power Normal Lv.MAX",
        format!(
            "{ap} damage<br>({dps} DPS)",
            ap = ap_max.to_formatted_string(&Locale::en),
            dps = get_formatted_float(dps_max, 2)
        ),
    ));

    t.push_params(TemplateParameter::new(
        "Attack type Normal",
        match stats.attack.aoe {
            AreaOfEffect::SingleAttack => "Single Attack",
            AreaOfEffect::AreaAttack => "Area Attack",
        },
    ));

    let mut abilities =  get_multihit_ability(&cat.unitlevel, stats, level) ;
    abilities.extend(get_range_ability(&stats.attack.hits));
    abilities.extend(get_pure_abilities(stats));

    t.push_params(TemplateParameter::new(
        "Special Ability Normal",
        abilities.join("<br>\n"),
    ));

    println!("{t}");
}

fn get_multihit_ability(scaling: &UnitLevelRaw, stats: &CatFormStats, level: u8) -> Vec<String> {
    let mut inherent = vec![];

    fn write_hit(buf: &mut String, hit: &AttackHit, level: u8, scale: &UnitLevelRaw) {
        buf.write_formatted(&scale.get_stat_at_level(hit.damage, level), &Locale::en)
            .infallible_write();
        buf.write_str(" at ").infallible_write();
        let (fore_f, fore_s) = time_repr(hit.foreswing.into());
        write!(buf, "{fore_f}f <sup>{fore_s}s</sup>").infallible_write();
    }

    let multihit = match &stats.attack.hits {
        AttackHits::Single(_) => None,
        AttackHits::Double([h1, h2]) => {
            let mut buf = "[[Special Abilities#Multi-Hit|Multi-Hit]] (".to_string();

            write_hit(&mut buf, &h1, level, scaling);
            buf.write_str(", ").infallible_write();
            write_hit(&mut buf, &h2, level, scaling);
            buf.write_str(")").infallible_write();

            Some(buf)
        }
        AttackHits::Triple([h1, h2, h3]) => {
            let mut buf = "[[Special Abilities#Multi-Hit|Multi-Hit]] (".to_string();

            write_hit(&mut buf, &h1, level, scaling);
            buf.write_str(", ").infallible_write();
            write_hit(&mut buf, &h2, level, scaling);
            buf.write_str(", ").infallible_write();
            write_hit(&mut buf, &h3, level, scaling);
            buf.write_str(")").infallible_write();

            Some(buf)
        }
    };

    inherent.extend(multihit);
    inherent
}
