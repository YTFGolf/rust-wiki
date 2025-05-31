use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::config::Config,
    wikitext::{
        template::{Template, TemplateParameter},
        wiki_utils::get_precision_f,
    },
};
use num_format::{Locale, WriteFormatted};
use std::{cmp::max, fmt::Write};

fn plural<'a>(amt: u16, single: &'a str, plural: &'a str) -> &'a str {
    if amt == 1 { single } else { plural }
}

fn plural_f<'a>(amt: u32, single: &'a str, plural: &'a str) -> &'a str {
    if amt == 30 { single } else { plural }
}

fn write_seconds(buf: &mut String, time_f: u32) {
    let time_s = f64::from(time_f) / 30.0;
    assert!(time_s < 1_000.0, "Amount of seconds is above 1,000!");
    let precision = get_precision_f(time_f);
    write!(buf, "{time_s:.precision$}").unwrap();
}

fn seconds_repr(time_f: u32) -> String {
    let mut buf = String::new();
    write_seconds(&mut buf, time_f);
    buf
}

/// `(frames, seconds)`
fn time_repr(time_f: u32) -> (String, String) {
    let f = {
        let mut buf = String::new();
        buf.write_formatted(&time_f, &Locale::en).unwrap();
        buf
    };

    let s = seconds_repr(time_f);

    (f, s)
}

fn get_template(cat: Cat) {
    let forms = &cat.forms;
    let stats = &forms.stats[0];
    let anims = &forms.anims[0];

    let mut t = Template::named("Cat Stats");

    let foreswing = stats.attack.hits.foreswing();
    let attack_length = stats.attack.hits.attack_length();
    let backswing = anims.length() - attack_length;
    let frequency = attack_length + max(2 * stats.attack.tba - 1, backswing);

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
        stats.speed.to_string(),
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

    t.push_params(TemplateParameter::new("Hp Normal Lv.MAX", "?"));
    t.push_params(TemplateParameter::new("Atk Power Normal Lv.MAX", "?"));
    t.push_params(TemplateParameter::new("Attack type Normal", "?"));
    t.push_params(TemplateParameter::new("Special Ability Normal", "?"));

    /*
    'Normal Form name': names[0],
    'Hp Normal': f"{baseStats['hp']:,} HP",
    'Atk Power Normal': f"{baseStats['ap']:,} damage<br>({baseStats['dps']:,} DPS)",
    'Atk Range Normal': f"{baseStats['rng']:,}",

    'Attack Frequency Normal': f"{baseStats['freq']}f <sub>{f_to_s(baseStats['freq'])} seconds</sub>",
    'Attack Animation Normal': f"{baseStats['fore']}f <sup>{f_to_s(baseStats['fore'])}s</sup><br>({baseStats['back']}f <sup>{f_to_s(baseStats['back'])}s</sup> backswing)",
    'Recharging Time Normal': f"{baseStats['rch']} ~ {baseStats['rchT']} seconds",
    'Hp Normal Lv.MAX': f"{cat.getStat(0, 30, 'hp'):,} HP",
    'Atk Power Normal Lv.MAX': f"{cat.getStat(0, 30, 'ap'):,} damage<br>({cat.getStat(0, 30, 'dps'):,} DPS)",
    'Attack type Normal': baseStats['atkType'],
    'Special Ability Normal': cat.abilityDesc(0, 0),
     */

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
