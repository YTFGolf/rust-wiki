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
