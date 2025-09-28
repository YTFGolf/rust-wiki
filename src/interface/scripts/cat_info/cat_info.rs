//! Script for cat info.

use crate::{
    game_data::cat::{
        parsed::{
            cat::{Cat, CatDataError},
            unitbuy::{AncientEggInfo, EvolutionType, Rarity},
        },
        raw::{
            combo::{ComboData, CombosDataContainer},
            combo_local::ComboNames,
            desc::get_cat_descriptions,
        },
    },
    interface::{
        config::{Config, cat_config::StatsTemplateVersion},
        error_handler::InfallibleWrite,
        scripts::cat_info::{
            costs::price_cost,
            form_util::CatForm,
            stats::stats_template::{manual::stats_manual, ver_0o1::stats_0o1},
            upgrade_cost::upgrade_cost,
        },
    },
    wikitext::{
        page::Page,
        section::Section,
        template::{Template, TemplateParameter},
    },
};
use std::fmt::Write;

/// "Description" template.
fn get_descs(cat: &Cat, config: &Config) -> Template {
    type P = TemplateParameter;

    let mut descs = Template::new(
        "Description",
        vec![
            P::new("Mode", "Cat"),
            P::new("Number", cat.id.to_string()),
            P::new("Type", cat.unitbuy.misc.rarity.category()),
        ],
    );

    {
        let id = cat.id;
        let (form1, form2) = match cat.unitbuy.misc.egg_info {
            AncientEggInfo::None => (format!("Uni{id:03} f00.png"), format!("Uni{id:03} c00.png")),
            AncientEggInfo::Egg { normal, evolved } => (
                format!("Uni{normal:03} m00.png"),
                format!("Uni{evolved:03} m01.png"),
            ),
        };

        descs.push_params(P::new("Image1", form1));
        if cat.forms.amt_forms >= 2 {
            descs.push_params(P::new("Image2", form2));
        }
        if cat.forms.amt_forms >= 3 {
            descs.push_params(P::new("Image3", format!("Uni{id:03} s00.png")));
        }
        if cat.forms.amt_forms >= 4 {
            descs.push_params(P::new("Image4", format!("Uni{id:03} u00.png")));
        }
    }

    for (num, desc) in get_cat_descriptions(cat.id, config.version.en())
        .into_iter()
        .flatten()
        .take(cat.forms.amt_forms)
        .enumerate()
    {
        descs.push_params([
            P::new(format!("Name{}", num + 1), desc.name().to_string()),
            P::new(format!("Desc{}", num + 1), desc.lines()),
        ]);
    }

    for (num, desc) in get_cat_descriptions(cat.id, config.version.jp())
        .into_iter()
        .flatten()
        .take(cat.forms.amt_forms)
        .enumerate()
    {
        descs.push_params([
            P::new(format!("Name{} (JP)", num + 1), desc.name().to_string()),
            P::new(format!("Desc{} (JP)", num + 1), desc.lines()),
        ]);
    }

    for (num, desc) in get_cat_descriptions(cat.id, config.version.tw())
        .into_iter()
        .flatten()
        .take(cat.forms.amt_forms)
        .enumerate()
    {
        descs.push_params([
            P::new(format!("Name{} (TW)", num + 1), desc.name().to_string()),
            P::new(format!("Desc{} (TW)", num + 1), desc.lines()),
        ]);
    }

    for (num, desc) in get_cat_descriptions(cat.id, config.version.kr())
        .into_iter()
        .flatten()
        .take(cat.forms.amt_forms)
        .enumerate()
    {
        descs.push_params([
            P::new(format!("Name{} (KR)", num + 1), desc.name().to_string()),
            P::new(format!("Desc{} (KR)", num + 1), desc.lines()),
        ]);
    }

    descs
}

/// Page introduction.
fn intro(cat: &Cat) -> Section {
    let first_name = CatForm::Normal.name(cat.id);
    let rarity = cat.unitbuy.misc.rarity;
    let category = rarity.category();

    let an = if rarity == Rarity::UberRare {
        "an"
    } else {
        "a"
    };

    let mut buf = format!("'''{first_name}''' is {an} {category}.");

    let update = cat.unitbuy.misc.update_released;
    let u = {
        if update <= 0 {
            None
        } else {
            let mut buf = format!(
                "{major}.{minor}",
                major = update / 10000,
                minor = update / 100 % 100
            );
            let patch = update % 100;
            if patch != 0 {
                write!(buf, ",{patch}").infallible_write();
            }

            Some(buf)
        }
    };

    if let Some(ver) = u {
        write!(
            buf,
            " It was added in [[Version {ver} Update|Version {ver}]]."
        )
        .infallible_write();
    }

    Section::blank(buf)
}

/// Write representation of evolution type to "Evolves into {name}" line.
fn write_evolution_type(buf: &mut String, et: &EvolutionType) {
    match et {
        EvolutionType::Levels { level } => write!(buf, " at level {level}.").infallible_write(),
        EvolutionType::Other => *buf += " via ???.",
        EvolutionType::Catfruit(evol) => {
            // assume that catfruit applies
            let fruit = "[[Catfruit]]";
            let level = evol.level_required;
            let xp = if evol.xp_cost > 0 { " and XP" } else { "" };
            write!(buf, " at level {level} using {fruit}{xp}.").infallible_write()
        }
    }
}

/// "Evolution" section.
fn evolution(cat: &Cat) -> Section {
    const TITLE: &str = "Evolution";

    if cat.forms.amt_forms <= 1 {
        return Section::h2(TITLE, "-");
    }

    let mut buf = String::new();

    let name = CatForm::Evolved.name(cat.id);
    // let evol = cat.unitbuy._uk21
    write!(buf, "Evolves into '''{name}''' at level 10.").infallible_write();

    let t = match &cat.unitbuy.true_evol {
        None => return Section::h2(TITLE, buf),
        Some(t) => t,
    };
    let name = CatForm::True.name(cat.id);
    write!(buf, "\n\nEvolves into '''{name}'''").infallible_write();
    write_evolution_type(&mut buf, &t.etype);

    let u = match &cat.unitbuy.ultra_evol {
        None => return Section::h2(TITLE, buf),
        Some(u) => u,
    };
    let name = CatForm::Ultra.name(cat.id);
    write!(buf, "\n\nEvolves into '''{name}'''").infallible_write();
    write_evolution_type(&mut buf, &u.etype);

    Section::h2(TITLE, buf)
}

/// "Cat Appearance" template.
fn appearance(cat: &Cat) -> Template {
    type P = TemplateParameter;
    let id = cat.id;

    // functional programming sure is beautiful
    Template::named("Cat Appearance")
        .add_params(P::new("Cat Unit Number", id.to_string()))
        .add_params(P::new("cat category", cat.unitbuy.misc.rarity.category()))
        .add_params(P::new("Normal Form name", CatForm::Normal.name(id)))
        .add_params(
            CatForm::Evolved
                .name_option(id)
                .map(|n| P::new("Evolved Form name", n)),
        )
        .add_params(
            CatForm::True
                .name_option(id)
                .map(|n| P::new("True Form name", n)),
        )
        .add_params(
            CatForm::Ultra
                .name_option(id)
                .map(|n| P::new("Ultra Form name", n)),
        )
        .add_params((cat.forms.amt_forms >= 1).then(|| P::new("image1", format!("{id:03} 1.png"))))
        .add_params((cat.forms.amt_forms >= 2).then(|| P::new("image2", format!("{id:03} 2.png"))))
        .add_params((cat.forms.amt_forms >= 3).then(|| P::new("image3", format!("{id:03} 3.png"))))
        .add_params((cat.forms.amt_forms >= 4).then(|| P::new("image4", format!("{id:03} 4.png"))))
}

fn fmt_combo(i: usize, combo: &ComboData, config: &Config) -> String {
    // let mut buf = String::from("{{CatCombo|");

    // let combo_name = todo!();
    // buf += combo_name;
    // let combo_effect = todo!();
    // buf += combo_effect;

    // for cat in combo.cats {
    //     buf += "|" + catname;
    // }

    // buf += "|jpname=";
    // buf += jpname;
    // buf += "}}";

    // // format!("Playing Slayer|"Strong" Effect UP (Sm)|Brave Cat|Dioramos|jpname=龍退治ごっこ}}}}")
    // buf
    let en_names = config.version.en().get_cached_file::<ComboNames>();
    log::warn!("this is just a demonstration");
    en_names.combo_name(i).unwrap().to_string()
}

fn combos(cat: &Cat, config: &Config) -> Option<Section> {
    let combo_container = config
        .version
        .current_version()
        .get_cached_file::<CombosDataContainer>();

    let mut has_cat = combo_container.by_cat_id(cat.id.try_into().unwrap());

    let first = has_cat.next()?;
    let mut buf = String::from("{{Combos\n");

    writeln!(buf, "|{fmt}", fmt = fmt_combo(first.0, first.1, config)).infallible_write();
    for rest in has_cat {
        writeln!(buf, "|{fmt}", fmt = fmt_combo(rest.0, rest.1, config)).infallible_write();
    }

    Some(Section::blank(buf + "}}"))
}

/// Get cat info.
pub fn get_info(wiki_id: u32, config: &Config) -> Result<Page, CatDataError> {
    let cat = Cat::from_wiki_id(wiki_id, &config.version)?;

    let mut page = Page::blank();

    page.push(intro(&cat));
    page.push(Section::blank(appearance(&cat).to_string()));
    page.push(evolution(&cat));
    page.push(Section::h2("Strategy/Usage", "-"));
    if let Some(combo_section) = combos(&cat, config) {
        page.push(combo_section);
    }

    page.push(Section::h2(
        "Description",
        get_descs(&cat, config).to_string(),
    ));
    page.push(price_cost(&cat, config));
    page.push(upgrade_cost(&cat, config));

    let stats = match config.cat_info.stats_template_version {
        StatsTemplateVersion::Current | StatsTemplateVersion::Ver0o1 => stats_0o1(&cat, config),
        StatsTemplateVersion::Manual => stats_manual(&cat, config),
    };
    page.push(Section::h2("Stats", stats.to_string()));

    Ok(page)
}

/*
talents
combos
*/
