use super::stats::abilities::{
    pure_abilities::get_pure_abilities, range_abilities::get_range_ability,
};
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

/// Includes LD, Omni and any abilities, but does not include multhit.
fn get_abilities(abilities: &mut Vec<String>, stats: &CatFormStats) {
    abilities.extend(get_range_ability(&stats.attack.hits));
    abilities.extend(get_pure_abilities(stats));
}

fn get_template(cat: Cat) {
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

    let mut abilities = {
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

                write_hit(&mut buf, &h1, level, &cat.unitlevel);
                buf.write_str(", ").infallible_write();
                write_hit(&mut buf, &h2, level, &cat.unitlevel);
                buf.write_str(")").infallible_write();

                Some(buf)
            }
            AttackHits::Triple([h1, h2, h3]) => {
                let mut buf = "[[Special Abilities#Multi-Hit|Multi-Hit]] (".to_string();

                write_hit(&mut buf, &h1, level, &cat.unitlevel);
                buf.write_str(", ").infallible_write();
                write_hit(&mut buf, &h2, level, &cat.unitlevel);
                buf.write_str(", ").infallible_write();
                write_hit(&mut buf, &h3, level, &cat.unitlevel);
                buf.write_str(")").infallible_write();

                Some(buf)
            }
        };

        inherent.extend(multihit);
        inherent
    };
    get_abilities(&mut abilities, stats);
    t.push_params(TemplateParameter::new(
        "Special Ability Normal",
        abilities.join("<br>\n"),
    ));

    println!("{t}");
}

/// Do thing.
pub fn do_thing(wiki_id: u32, config: &Config) {
    println!("{wiki_id:?} {config:?}");
    get_template(Cat::from_wiki_id(wiki_id, &config.version).unwrap());
}

/*
talents
combos
desc (will need to make other parts better)
*/

/*
standardStats = Template('Cat Stats', {
    'Evolved Form name': names[1],
    'Hp Evolved': f"{cat.getStat(1, 30, 'hp'):,} HP",
    'Atk Power Evolved': f"{cat.getStat(1, 30, 'ap'):,} damage<br>({cat.getStat(1, 30, 'dps'):,} DPS)",
    'Atk Range Evolved': f"{normStats['rng']:,}",
    'Attack Frequency Evolved': f"{normStats['freq']}f <sub>{f_to_s(normStats['freq'])} seconds</sub>",
    'Movement Speed Evolved': normStats['spd'],
    'Knockback Evolved': f"{normStats['kb']} time{'s' if normStats['kb'] > 1 else ''}",
    'Attack Animation Evolved': f"{normStats['fore']}f <sup>{f_to_s(normStats['fore'])}s</sup><br>({normStats['back']}f <sup>{f_to_s(normStats['back'])}s</sup> backswing)",
    'Recharging Time Evolved': f"{normStats['rch']} ~ {normStats['rchT']} seconds",
    'Attack type Evolved': normStats['atkType'],
    'Special Ability Evolved': cat.abilityDesc(0, 1)
})
if cat.hasTrue:
    trueStats = cat.getBaseStats(2)
    standardStats.addArgs({
        'True Form name': names[2],
        'Hp True': f"{cat.getStat(2, 30, 'hp'):,} HP",
        'Atk Power True': f"{cat.getStat(2, 30, 'ap'):,} damage<br>({cat.getStat(2, 30, 'dps'):,} DPS)",
        'Atk Range True': f"{trueStats['rng']:,}",
        'Attack Frequency True': f"{trueStats['freq']}f <sub>{f_to_s(trueStats['freq'])} seconds</sub>",
        'Movement Speed True': trueStats['spd'],
        'Knockback True': f"{trueStats['kb']} time{'s' if trueStats['kb'] > 1 else ''}",
        'Attack Animation True': f"{trueStats['fore']}f <sup>{f_to_s(trueStats['fore'])}s</sup><br>({trueStats['back']}f <sup>{f_to_s(trueStats['back'])}s</sup> backswing)",
        'Recharging Time True': f"{trueStats['rch']} ~ {trueStats['rchT']} seconds",
        'Attack type True': trueStats['atkType'],
        'Special Ability True': cat.abilityDesc(0, 2)
    })
if cat.hasUltra:
    ultraStats = cat.getBaseStats(3)
    standardStats.addArgs({
        'Ultra Form name': names[3],
        'Hp Ultra': f"{cat.getStat(3, 60, 'hp'):,} HP",
        'Atk Power Ultra': f"{cat.getStat(3, 60, 'ap'):,} damage<br>({cat.getStat(3, 60, 'dps'):,} DPS)",
        'Atk Range Ultra': f"{ultraStats['rng']:,}",
        'Attack Frequency Ultra': f"{ultraStats['freq']}f <sub>{f_to_s(ultraStats['freq'])} seconds</sub>",
        'Movement Speed Ultra': ultraStats['spd'],
        'Knockback Ultra': f"{ultraStats['kb']} time{'s' if ultraStats['kb'] > 1 else ''}",
        'Attack Animation Ultra': f"{ultraStats['fore']}f <sup>{f_to_s(ultraStats['fore'])}s</sup><br>({ultraStats['back']}f <sup>{f_to_s(ultraStats['back'])}s</sup> backswing)",
        'Recharging Time Ultra': f"{ultraStats['rch']} ~ {ultraStats['rchT']} seconds",
        'Attack type Ultra': ultraStats['atkType'],
        'Special Ability Ultra': cat.abilityDesc(0, 3)
    })
*/
