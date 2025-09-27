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
        scripts::cat_info::{
            form_util::CatForm,
            stats::stats_template::{manual::stats_manual, ver_0o1::stats_0o1},
        },
    },
    wikitext::{
        page::Page,
        section::Section,
        template::{Template, TemplateParameter},
    },
};
use num_format::{Locale, ToFormattedString};

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

fn fmt_cost(chap_1: u16) -> String {
    format!(
        "*Chapter 1: {}¢\n\
        *Chapter 2: {}¢\n\
        *Chapter 3: {}¢",
        chap_1.to_formatted_string(&Locale::en),
        (chap_1 * 15 / 10).to_formatted_string(&Locale::en),
        (chap_1 * 2).to_formatted_string(&Locale::en)
    )
}

fn cost(cat: &Cat, _config: &Config) -> Section {
    const TITLE: &str = "Cost";
    let mut costs: Vec<(u16, Vec<usize>)> = vec![];
    for (i, (stats, _)) in cat.forms.iter().enumerate() {
        match costs.iter().position(|c| c.0 == stats.price) {
            None => costs.push((stats.price, vec![i])),
            Some(j) => costs[j].1.push(i),
        }
    }

    assert!(!costs.is_empty());

    if costs.len() == 1 {
        let first = costs
            .iter()
            .next()
            .expect("already asserted costs is not empty");
        return Section::h2(TITLE, fmt_cost(first.0));
    }

    let mut costs_str = Page::blank();
    for (cost, forms) in costs {
        let title = forms
            .iter()
            .map(|f| {
                CatForm::from_repr(*f)
                    .expect("cat form should not fail")
                    .as_str()
            })
            .collect::<Vec<_>>()
            .join("/")
            + " Form";
        costs_str.push(Section::h3(title, fmt_cost(cost)));
    }

    Section::h2(TITLE, costs_str.to_string())
}

/// Get cat info.
pub fn get_info(wiki_id: u32, config: &Config) -> Result<Page, CatDataError> {
    let cat = Cat::from_wiki_id(wiki_id, &config.version)?;

    let mut page = Page::blank();

    page.push(Section::h2(
        "Description",
        get_descs(&cat, config).to_string(),
    ));

    page.push(cost(&cat, config));

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
desc
*/

#[cfg(test)]
mod tests {
    #[test]
    fn basic_cost() {
        // id = 0
        todo!()
    }

    #[test]
    fn cost_not_even() {
        // moneko
        todo!()
    }

    #[test]
    fn cost_varies_by_form() {
        // aer, 361
        todo!()
    }

    #[test]
    fn cost_triple_unique() {
        // cosmo, 135
        todo!()
        /*
        ==Cost==
        ===Normal Form===
        *Chapter 1: 555¢
        *Chapter 2: 832¢
        *Chapter 3: 1,110¢

        ===Evolved/True Form===
        *Chapter 1: 3,900¢
        *Chapter 2: 5,850¢
        *Chapter 3: 7,800¢

        ===Ultra Form===
        *Chapter 1: 3,000¢
        *Chapter 2: 4,500¢
        *Chapter 3: 6,000¢
        {{Upgrade Cost|UR}}
         */
    }

    #[test]
    fn cost_returns() {
        // kaguya, 138
        todo!()
    }
}
