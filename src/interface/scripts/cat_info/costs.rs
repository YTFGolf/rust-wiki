//! Deals with the "Cost" section.

use crate::{
    game_data::cat::parsed::cat::Cat,
    interface::{config::Config, scripts::cat_info::form_util::CatForm},
    wikitext::{page::Page, section::Section},
};
use num_format::{Locale, ToFormattedString};

fn fmt_cost(chap_1: u16) -> String {
    format!(
        "*Chapter 1: {}¢\n\
        *Chapter 2: {}¢\n\
        *Chapter 3: {}¢",
        chap_1.to_formatted_string(&Locale::en),
        (chap_1 + chap_1 / 2).to_formatted_string(&Locale::en),
        (chap_1 * 2).to_formatted_string(&Locale::en)
    )
}

/// In-battle "Cost" section.
pub fn deploy_cost(cat: &Cat, _config: &Config) -> Section {
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
        let first = costs.first().expect("already asserted costs is not empty");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn basic_cost() {
        let cat = Cat::from_wiki_id(0, &TEST_CONFIG.version).unwrap();
        let sect = deploy_cost(&cat, &TEST_CONFIG).to_string();

        assert_eq!(
            sect,
            "==Cost==\n\
            *Chapter 1: 50¢\n\
            *Chapter 2: 75¢\n\
            *Chapter 3: 100¢"
        );
    }

    #[test]
    fn cost_not_even() {
        let moneko = Cat::from_wiki_id(16, &TEST_CONFIG.version).unwrap();
        let sect = deploy_cost(&moneko, &TEST_CONFIG).to_string();

        assert_eq!(
            sect,
            "==Cost==\n\
            *Chapter 1: 99¢\n\
            *Chapter 2: 148¢\n\
            *Chapter 3: 198¢"
        );
    }

    #[test]
    fn cost_varies_by_form() {
        let aer = Cat::from_wiki_id(361, &TEST_CONFIG.version).unwrap();
        let sect = deploy_cost(&aer, &TEST_CONFIG).to_string();

        assert_eq!(
            sect,
            "==Cost==\n\
            ===Normal Form===\n\
            *Chapter 1: 720¢\n\
            *Chapter 2: 1,080¢\n\
            *Chapter 3: 1,440¢\n\
            \n\
            ===Evolved/True/Ultra Form===\n\
            *Chapter 1: 2,620¢\n\
            *Chapter 2: 3,930¢\n\
            *Chapter 3: 5,240¢"
        );
    }

    #[test]
    fn cost_triple_unique() {
        let cosmo = Cat::from_wiki_id(135, &TEST_CONFIG.version).unwrap();
        let sect = deploy_cost(&cosmo, &TEST_CONFIG).to_string();

        assert_eq!(
            sect,
            "==Cost==\n\
            ===Normal Form===\n\
            *Chapter 1: 555¢\n\
            *Chapter 2: 832¢\n\
            *Chapter 3: 1,110¢\n\
            \n\
            ===Evolved/True Form===\n\
            *Chapter 1: 3,900¢\n\
            *Chapter 2: 5,850¢\n\
            *Chapter 3: 7,800¢\n\
            \n\
            ===Ultra Form===\n\
            *Chapter 1: 3,000¢\n\
            *Chapter 2: 4,500¢\n\
            *Chapter 3: 6,000¢"
        );
    }

    #[test]
    fn cost_returns() {
        let kaguya = Cat::from_wiki_id(138, &TEST_CONFIG.version).unwrap();
        let sect = deploy_cost(&kaguya, &TEST_CONFIG).to_string();

        assert_eq!(
            sect,
            "==Cost==\n\
            ===Normal/Ultra Form===\n\
            *Chapter 1: 400¢\n\
            *Chapter 2: 600¢\n\
            *Chapter 3: 800¢\n\
            \n\
            ===Evolved/True Form===\n\
            *Chapter 1: 3,200¢\n\
            *Chapter 2: 4,800¢\n\
            *Chapter 3: 6,400¢"
        );
    }
}
