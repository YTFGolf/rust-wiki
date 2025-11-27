//! Deals with the CatNav template.

use crate::{wiki_data::cat_data::CAT_DATA, wikitext::section::Section};

/// CatNav template.
pub fn cat_nav(id: u32) -> Section {
    let mut nav = String::from("----\n{{CatNav|");

    let mut prev_id = id;
    while prev_id > 0 {
        prev_id -= 1;
        let cat = CAT_DATA.get_cat(prev_id);

        if ["Iron Wall Cat"].contains(&cat.normal.as_str())
            || ["Special Abilities#Conjure"].contains(&cat.page.as_str())
        {
            continue;
        }

        nav += &cat.normal;
        break;
    }

    nav += "|";

    let mut next_id = id as usize;
    loop {
        next_id += 1;
        let cat = match CAT_DATA.try_get_cat(next_id) {
            Some(c) => c,
            None => break,
        };

        if ["Iron Wall Cat"].contains(&cat.normal.as_str())
            || ["Special Abilities#Conjure"].contains(&cat.page.as_str())
        {
            continue;
        }

        nav += &cat.normal;
        break;
    }

    nav += "}}\n----";

    Section::blank(nav)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TEST_CONFIG, game_data::cat::parsed::cat::Cat};

    #[test]
    fn cat_nav_first() {
        let cat = Cat::from_wiki_id(0, &TEST_CONFIG.version).unwrap();
        let sect = cat_nav(cat.id).to_string();

        const TARGET: &str = concat!(
            "----\n",
            "{{CatNav||Tank Cat}}",
            // comment to avoid rustfmt
            "\n----"
        );
        assert_eq!(sect, TARGET);
    }

    #[test]
    fn cat_nav_spirit_front() {
        let izanagi = Cat::from_wiki_id(731, &TEST_CONFIG.version).unwrap();
        // has spirit
        let sect = cat_nav(izanagi.id).to_string();

        const TARGET: &str = concat!(
            "----\n",
            "{{CatNav|Ancient Egg: N204|Pegasa}}",
            // comment to avoid rustfmt
            "\n----"
        );
        assert_eq!(sect, TARGET);
    }

    #[test]
    fn cat_nav_spirit_back() {
        let pegasa = Cat::from_wiki_id(733, &TEST_CONFIG.version).unwrap();
        let sect = cat_nav(pegasa.id).to_string();

        const TARGET: &str = concat!(
            "----\n",
            "{{CatNav|Daybreaker Izanagi|Principal Cat}}",
            // comment to avoid rustfmt
            "\n----"
        );
        assert_eq!(sect, TARGET);
    }

    #[test]
    fn cat_nav_spirit_both() {
        let newton = Cat::from_wiki_id(801, &TEST_CONFIG.version).unwrap();
        let sect = cat_nav(newton.id).to_string();

        const TARGET: &str = concat!(
            "----\n",
            "{{CatNav|Mighty Morta-Loncha|Sonic}}",
            // comment to avoid rustfmt
            "\n----"
        );
        assert_eq!(sect, TARGET);
    }
}
