//! Script for cat info.

use crate::{
    game_data::cat::{
        parsed::{
            cat::{Cat, CatDataError},
            unitbuy::AncientEggInfo,
        },
        raw::desc::get_cat_descriptions,
    },
    interface::{
        config::{Config, cat_config::StatsTemplateVersion},
        scripts::cat_info::stats::template::{current::get_template, manual::get_old_template},
    },
    wikitext::template::{Template, TemplateParameter},
};

fn get_descs(cat: &Cat, config: &Config) -> Template {
    type P = TemplateParameter;

    let mut descs = Template::new(
        "Description",
        vec![
            P::new("Mode", "Cat"),
            P::new("Number", cat.id.to_string()),
            P::new(
                "Type",
                format!(
                    "[[:Category:{t} Cats|{t} Cat]]",
                    t = cat.unitbuy.misc.rarity.as_str()
                ),
            ),
        ],
    );

    {
        let id = cat.id;
        let (form1, form2) = match cat.unitbuy.misc.egg_info {
            AncientEggInfo::None => (format!("Uni{id:03} f00.png"), format!("Uni{id:03} c00.png")),
            AncientEggInfo::Egg { normal, evolved } => (
                format!("Uni{id:03} m{normal:02}.png"),
                format!("Uni{id:03} m{evolved:02}.png"),
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

    for (num, desc) in get_cat_descriptions(cat.id, &config.version.en())
        .take(cat.forms.amt_forms)
        .enumerate()
    {
        descs.push_params([
            P::new(format!("Name{}", num + 1), desc.name().to_string()),
            P::new(format!("Desc{}", num + 1), desc.lines()),
        ]);
    }

    for (num, desc) in get_cat_descriptions(cat.id, &config.version.jp())
        .take(cat.forms.amt_forms)
        .enumerate()
    {
        descs.push_params([
            P::new(format!("Name{} (JP)", num + 1), desc.name().to_string()),
            P::new(format!("Desc{} (JP)", num + 1), desc.lines()),
        ]);
    }

    for (num, desc) in get_cat_descriptions(cat.id, &config.version.tw())
        .take(cat.forms.amt_forms)
        .enumerate()
    {
        descs.push_params([
            P::new(format!("Name{} (TW)", num + 1), desc.name().to_string()),
            P::new(format!("Desc{} (TW)", num + 1), desc.lines()),
        ]);
    }

    for (num, desc) in get_cat_descriptions(cat.id, &config.version.kr())
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

/// Get cat info.
pub fn get_info(wiki_id: u32, config: &Config) -> Result<String, CatDataError> {
    let cat = Cat::from_wiki_id(wiki_id, &config.version)?;

    if false {
        let descs = get_descs(&cat, config);
        panic!("{descs}");
    }

    let stats = match config.cat_info.stats_template_version {
        StatsTemplateVersion::Current => get_template(&cat).to_string(),
        StatsTemplateVersion::Manual => get_old_template(&cat).to_string(),
    };

    Ok(stats)
}

/*
talents
combos
desc
*/
