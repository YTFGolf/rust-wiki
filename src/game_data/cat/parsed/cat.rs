//! Deals with cat data.

use super::{
    anim::{AnimData, get_anims},
    cat_stats::CatStats,
    unitbuy::{AncientEggInfo, UnitBuyData},
};
use crate::game_data::{
    cat::raw::{stats::read_data_file, unitbuy::UnitBuyContainer, unitexp::Levelling},
    version::Version,
};

#[derive(Debug)]
/// Data about individual forms of the cat.
pub struct CatForms {
    /// Amount of forms the cat has.
    ///
    /// Guaranteed by assertion that `stats` and `anims` will have at least this
    /// many forms.
    pub amt_forms: usize,
    /// Stats per form.
    pub stats: Vec<CatStats>,
    /// Animation data for each form.
    pub anims: Vec<AnimData>,
    // desc
}

#[derive(Debug)]
/// Parsed cat object.
pub struct Cat {
    /// CRO id.
    pub id: u32,
    /// Cat's forms.
    pub forms: CatForms,
    /// Data from `unitbuy.csv`.
    pub unitbuy: UnitBuyData,
    /// Data from `unitexp.csv`.
    pub unitexp: Levelling,
    // growth curve
    // talents
    // evolutions
    // combos
}

#[derive(Debug)]
/// Error when getting cat data.
pub enum CatDataError {
    /// No data in `unitbuy.csv`. Almost certainly means that the unit does not
    /// exist in the current version.
    UnitBuyNotFound,
}

impl Cat {
    /// Get cat from wiki id.
    pub fn from_wiki_id(wiki_id: u32, version: &Version) -> Result<Self, CatDataError> {
        type E = CatDataError;
        let id = wiki_id;

        let unitbuy = version.get_cached_file::<UnitBuyContainer>();
        let unitbuy = UnitBuyData::from_unitbuy(unitbuy.get_unit(id).ok_or(E::UnitBuyNotFound)?);

        let unitexp = Levelling::from_id(id);

        let is_summon = unitbuy.misc.is_summon();
        let has_true = unitbuy.true_evol.is_some();
        let has_ultra = unitbuy.ultra_evol.is_some();
        let egg_data = &unitbuy.misc.egg_info;

        let amt_forms = Self::get_amt_forms(id, is_summon, has_true, has_ultra);
        let forms = Self::get_forms(id, version, amt_forms, egg_data);

        Ok(Self {
            id,
            forms,
            unitbuy,
            unitexp,
        })
    }

    fn get_amt_forms(id: u32, is_summon: bool, has_true: bool, has_ultra: bool) -> usize {
        match id {
            339 | 673 => 1,
            // iron wall, cheetah
            _ if is_summon => 1,
            _ => 2 + has_true as usize + has_ultra as usize,
        }
    }

    fn get_forms(
        id: u32,
        version: &Version,
        amt_forms: usize,
        egg_data: &AncientEggInfo,
    ) -> CatForms {
        let stats = Self::get_stats(id, version).collect::<Vec<_>>();
        let anims = get_anims(id, version, amt_forms, egg_data).unwrap();

        assert!(stats.len() >= amt_forms);
        assert!(anims.len() >= amt_forms);

        CatForms {
            amt_forms,
            stats,
            anims,
        }
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
    use crate::config::TEST_CONFIG;

    #[test]
    #[ignore]
    fn test_all() {
        type E = CatDataError;
        let version = TEST_CONFIG.version.jp();
        for id in 0..u32::MAX {
            if matches!(id, (740..=745) | 788 | 810) {
                continue;
            }
            match Cat::from_wiki_id(id, version) {
                Ok(_) => (),
                Err(E::UnitBuyNotFound) => break,
            }
        }
    }
}
