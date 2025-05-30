use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::config::Config,
    wikitext::template::{Template, TemplateParameter},
};

fn plural<'a>(amt: u16, single: &'a str, plural: &'a str) -> &'a str {
    if amt == 1 { single } else { plural }
}

fn get_template(cat: Cat) {
    let forms = &cat.forms;
    let stats = &forms.stats[0];
    let anims = &forms.anims[0];

    let params = [
        TemplateParameter::new("Attack Frequency Normal", "?"),
        TemplateParameter::new("Movement Speed Normal", stats.speed.to_string()),
        TemplateParameter::new(
            "Knockback Normal",
            format!(
                "{kb} {times}",
                kb = stats.kb,
                times = plural(stats.kb, "time", "times")
            ),
        ),
        TemplateParameter::new("Attack Animation Normal", "?"),
        TemplateParameter::new("Recharging Time Normal", "?"),
        TemplateParameter::new("Hp Normal Lv.MAX", "?"),
        TemplateParameter::new("Atk Power Normal Lv.MAX", "?"),
        TemplateParameter::new("Attack type Normal", "?"),
        TemplateParameter::new("Special Ability Normal", "?"),
    ];

    /*
    'Attack Frequency Normal': f"{baseStats['freq']}f <sub>{f_to_s(baseStats['freq'])} seconds</sub>",
    'Movement Speed Normal': baseStats['spd'],
    'Knockback Normal': f"{baseStats['kb']} time{'s' if baseStats['kb'] > 1 else ''}",
    'Attack Animation Normal': f"{baseStats['fore']}f <sup>{f_to_s(baseStats['fore'])}s</sup><br>({baseStats['back']}f <sup>{f_to_s(baseStats['back'])}s</sup> backswing)",
    'Recharging Time Normal': f"{baseStats['rch']} ~ {baseStats['rchT']} seconds",
    'Hp Normal Lv.MAX': f"{cat.getStat(0, 30, 'hp'):,} HP",
    'Atk Power Normal Lv.MAX': f"{cat.getStat(0, 30, 'ap'):,} damage<br>({cat.getStat(0, 30, 'dps'):,} DPS)",
    'Attack type Normal': baseStats['atkType'],
    'Special Ability Normal': cat.abilityDesc(0, 0),
     */

    let t = Template::named("Cat Stats").add_params(params);
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
    'Normal Form name': names[0],
    'Hp Normal': f"{baseStats['hp']:,} HP",
    'Atk Power Normal': f"{baseStats['ap']:,} damage<br>({baseStats['dps']:,} DPS)",
    'Atk Range Normal': f"{baseStats['rng']:,}",

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
