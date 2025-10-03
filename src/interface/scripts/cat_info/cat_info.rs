//! Script for cat info.

use crate::{
    game_data::cat::{
        parsed::{
            cat::{Cat, CatDataError},
            unitbuy::{
                EvolutionInfo, EvolutionType, Rarity, evolution_items::EvolutionItemVariant,
            },
        },
        raw::{desc::get_cat_descriptions, evolution_desc::EvolutionDescriptions},
    },
    interface::{
        config::{Config, cat_config::StatsTemplateVersion},
        error_handler::InfallibleWrite,
        scripts::cat_info::{
            combos::combos_section,
            costs::price_cost,
            form_util::CatForm,
            stats::stats_template::{
                manual::stats_manual, ver_0o1::stats_0o1, ver_0o2::stats_0o2, ver_1o0::stats_1o0,
            },
            talents::talents_section,
            upgrade_cost::upgrade_cost,
        },
    },
    wikitext::{
        page::Page,
        section::Section,
        template::{Template, TemplateParameter},
    },
};
use num_format::{Locale, ToFormattedString};
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
        let eggs = &cat.unitbuy.misc.egg_info;

        descs.push_params(P::new("Image1", CatForm::Normal.deploy_icon(id, eggs)));
        if cat.forms.amt_forms >= 2 {
            descs.push_params(P::new("Image2", CatForm::Evolved.deploy_icon(id, eggs)));
        }
        if cat.forms.amt_forms >= 3 {
            descs.push_params(P::new("Image3", CatForm::True.deploy_icon(id, eggs)));
        }
        if cat.forms.amt_forms >= 4 {
            descs.push_params(P::new("Image4", CatForm::Ultra.deploy_icon(id, eggs)));
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

fn catfruit_evolution(cat: &Cat, config: &Config) -> Option<Section> {
    const TITLE: &str = "Catfruit Evolution";
    type P = TemplateParameter;

    let Some(tf) = &cat.unitbuy.true_evol else {
        return None;
    };
    let tf = match &tf.etype {
        EvolutionType::Levels { .. } | EvolutionType::Other => return None,
        EvolutionType::Catfruit(catfruit_evolution) => catfruit_evolution,
    };

    let egg = &cat.unitbuy.misc.egg_info;

    let en_descriptions = config
        .version
        .en()
        .get_cached_file::<EvolutionDescriptions>();
    let jp_descriptions = config
        .version
        .jp()
        .get_cached_file::<EvolutionDescriptions>();

    let en_desc = en_descriptions.evolution_desc(cat.id.try_into().unwrap());
    let jp_desc = jp_descriptions
        .evolution_desc(cat.id.try_into().unwrap())
        .unwrap();
    let best_version = en_desc.unwrap_or(jp_desc);

    let mut t = Template::named(TITLE)
        .add_params(P::new("Evolved Name", CatForm::Evolved.name(cat.id)))
        .add_params(P::new(
            "Evolved Image",
            CatForm::Evolved.deploy_icon(cat.id, egg),
        ))
        .add_params(P::new("True Name", CatForm::True.name(cat.id)))
        .add_params(P::new("True Image", CatForm::True.deploy_icon(cat.id, egg)))
        .add_params(P::new("True Description", best_version.tf()));

    let mut to_extend = vec![];
    for (i, item) in tf.item_cost.iter().enumerate() {
        let value = match EvolutionItemVariant::from_repr(item.item_id.into()) {
            None => format!("zukan_{id}", id = item.item_id),
            Some(v) => match v {
                EvolutionItemVariant::Nothing => continue,
                v => format!("{v:?}"),
            },
        };

        let key = format!("Catfruit{}", i + 1);
        to_extend.push(P::new(
            "Quantity ".to_string() + &key,
            format!("x{}", item.item_amt),
        ));
        // need to do this first as using key to format
        t.push_params(P::new(key, value));
    }
    t.push_params(to_extend.into_iter());

    t.push_params(P::new(
        "Quantity XP",
        tf.xp_cost.to_formatted_string(&Locale::en),
    ));

    // --------------------
    //          UF
    // --------------------

    let uf = match &cat.unitbuy.ultra_evol {
        Some(EvolutionInfo {
            etype: EvolutionType::Catfruit(catfruit_evolution),
            ..
        }) => catfruit_evolution,
        _ => return Some(Section::h2(TITLE, t.to_string())),
    };

    t.push_params(P::new("Ultra Name", CatForm::Ultra.name(cat.id)));
    t.push_params(P::new(
        "Ultra Image",
        CatForm::Ultra.deploy_icon(cat.id, egg),
    ));

    let uf_desc = {
        let uf = best_version.uf();
        if uf == best_version.tf() {
            jp_desc.uf()
        } else {
            uf
        }
    };
    t.push_params(P::new("Ultra Description", uf_desc));

    let mut to_extend = vec![];
    for (i, item) in uf.item_cost.iter().enumerate() {
        let value = match EvolutionItemVariant::from_repr(item.item_id.into()) {
            None => format!("zukan_{id}", id = item.item_id),
            Some(v) => match v {
                EvolutionItemVariant::Nothing => continue,
                v => format!("{v:?}"),
            },
        };

        let key = format!("Catfruit{} Ultra", i + 1);
        to_extend.push(P::new(
            "Quantity ".to_string() + &key,
            format!("x{}", item.item_amt),
        ));
        // need to do this first as using key to format
        t.push_params(P::new(key, value));
    }
    t.push_params(to_extend.into_iter());

    t.push_params(P::new(
        "Quantity XP Ultra",
        uf.xp_cost.to_formatted_string(&Locale::en),
    ));

    Some(Section::h2(TITLE, t.to_string()))
}

/// Get cat info.
pub fn get_info(wiki_id: u32, config: &Config) -> Result<Page, CatDataError> {
    let cat = Cat::from_wiki_id(wiki_id, &config.version)?;

    let mut page = Page::blank();

    page.push(intro(&cat));
    page.push(Section::blank(appearance(&cat).to_string()));
    page.push(evolution(&cat));
    page.push(Section::h2("Strategy/Usage", "-"));
    if let Some(combo_section) = combos_section(&cat, config) {
        page.push(combo_section);
    }

    page.push(Section::h2(
        "Description",
        get_descs(&cat, config).to_string(),
    ));
    page.push(price_cost(&cat, config));
    page.push(upgrade_cost(&cat, config));

    let stats = match config.cat_info.stats_template_version {
        StatsTemplateVersion::Current | StatsTemplateVersion::Ver1o0 => stats_1o0(&cat, config),
        StatsTemplateVersion::Ver0o2 => stats_0o2(&cat, config),
        StatsTemplateVersion::Ver0o1 => stats_0o1(&cat, config),
        StatsTemplateVersion::Manual => stats_manual(&cat, config),
    };
    page.push(Section::h2("Stats", stats.to_string()));
    if let Some(cf_evo) = catfruit_evolution(&cat, config) {
        page.push(cf_evo);
    }
    if let Some(talents) = talents_section(&cat, config) {
        page.push(talents);
    }

    Ok(page)
}

/*
talents
*/
