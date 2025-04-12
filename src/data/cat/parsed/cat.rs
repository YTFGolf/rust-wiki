//! Deals with cat data.

#![allow(dead_code, unused_variables, missing_docs, unused_imports)]

use super::{
    cat_stats::CatStats,
    unitbuy::{self, AncientEggInfo, UnitBuyData},
};
use crate::data::{
    cat::raw::{stats::read_data_file, unitbuy::UnitBuyContainer, unitexp::Levelling},
    version::Version,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct AnimData {
    length: usize, // right now all that's needed is the length of the animation
}
impl AnimData {
    pub fn len(&self) -> usize {
        self.length
    }
}

#[derive(Debug)]
pub struct CatForms {
    amt_forms: usize,
    stats: Vec<CatStats>,
    anims: Vec<AnimData>,
    // anim
    // desc
}

#[derive(Debug)]
/// Parsed cat object.
pub struct Cat {
    /// CRO id.
    pub id: u32,
    /// Cat's forms.
    pub forms: CatForms,
    pub unitbuy: UnitBuyData,
    pub unitexp: Levelling,
    // growth curve
    // talents
    // evolutions
    // combos
}

impl Cat {
    /// Get cat from wiki id.
    pub fn from_wiki_id(wiki_id: u32, version: &Version) -> Self {
        let id = wiki_id;

        let unitbuy = version.get_cached_file::<UnitBuyContainer>();
        let unitbuy = UnitBuyData::from_unitbuy(unitbuy.get_unit(id).unwrap());

        let unitexp = Levelling::from_id(id);

        let has_true = unitbuy.true_evol.is_some();
        let has_ultra = unitbuy.ultra_evol.is_some();
        let egg_data = &unitbuy.misc.egg_info;

        let amt_forms = 2 + has_true as usize + has_ultra as usize;
        let forms = Self::get_forms(id, version, amt_forms, egg_data);

        Self {
            id,
            forms,
            unitbuy,
            unitexp,
        }
    }

    fn get_forms(
        id: u32,
        version: &Version,
        amt_forms: usize,
        egg_data: &AncientEggInfo,
    ) -> CatForms {
        let stats = Self::get_stats(id, version).collect();
        let anims = Self::get_anims(id, version, amt_forms, egg_data);

        CatForms {
            amt_forms,
            stats,
            anims,
        }
    }

    pub fn get_anims(
        wiki_id: u32,
        version: &Version,
        amt_forms: usize,
        egg_data: &AncientEggInfo,
    ) -> Vec<AnimData> {
        let (form1, form2) = match egg_data {
            AncientEggInfo::None => (
                format!("{wiki_id:03}_f02.maanim"),
                format!("{wiki_id:03}_c02.maanim"),
            ),
            AncientEggInfo::Egg { normal, evolved } => (
                format!("{normal:03}_m02.maanim"),
                format!("{evolved:03}_m02.maanim"),
            ),
        };

        let mut anims = [form1, form2]
            .iter()
            .map(|path| Self::get_anim_data(&path, version))
            .collect::<Vec<_>>();
        if amt_forms > 2 {
            let tf = format!("{wiki_id:03}_s02.maanim");
            anims.push(Self::get_anim_data(&tf, version))
        }
        if amt_forms > 3 {
            let uf = format!("{wiki_id:03}_u02.maanim");
            anims.push(Self::get_anim_data(&uf, version))
        }
        anims
    }

    fn get_anim_data(path: &str, version: &Version) -> AnimData {
        const ANIM_LINE_LEN: usize = 4;
        let qualified = version.get_file_path("ImageDataLocal").join(path);
        let count = BufReader::new(File::open(&qualified).unwrap())
            .lines()
            .filter_map(|line| {
                let line = line.as_ref().ok()?;
                let count = line.chars().filter(|c| *c == ',').count() + 1;
                // if has 4 items then has 3 commas
                if count != ANIM_LINE_LEN {
                    return None;
                };
                let frame_no = line.split(',').next()?;
                frame_no.parse::<usize>().ok()
            })
            .max()
            .unwrap();
        AnimData { length: count + 1 }
    }

    /// Get stats for each form.
    pub fn get_stats(wiki_id: u32, version: &Version) -> impl Iterator<Item = CatStats> {
        // get_stats(wiki_id + 1, version)
        let abs_id = wiki_id + 1;
        let file_name = format!("unit{abs_id:03}.csv");
        let combined_iter = read_data_file(&file_name, version);
        combined_iter.map(|combined| CatStats::from_combined(&combined))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::TEST_CONFIG, data::cat::raw::unitbuy::UnitBuyContainer};

    #[test]
    fn test_units() {
        let version = TEST_CONFIG.version.current_version();
        let unitbuy = version.get_cached_file::<UnitBuyContainer>();

        let test_units = [
            ("cat", 0),
            ("tank", 1),
            ("titan", 8),
            ("actress", 9),
            ("bahamut", 25),
            ("cancan", 32),
            ("dio", 177),
            ("metal", 200),
            ("dasli", 543),
            ("cat modoki", 626),
            ("sfeline", 643),
            ("courier", 658),
        ];
        for (name, id) in test_units {
            // println!("{name} ({id}) = {:?}\n", Cat::from_wiki_id(id, version));
            println!("{name} ({id}) = {:#?}\n", Cat::from_wiki_id(id, version));
        }
        todo!()
    }
}
