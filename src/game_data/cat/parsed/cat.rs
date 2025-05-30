//! Deals with cat data.

use super::{
    anim::{CatFormAnimData, get_anims},
    stats::form::CatFormStats,
    unitbuy::{AncientEggInfo, UnitBuy},
};
use crate::game_data::{
    cat::raw::{
        stats::read_data_file,
        unitbuy::UnitBuyContainer,
        unitexp::XPCostScale,
        unitlevel::{UnitLevelContainer, UnitLevelRaw},
    },
    version::{
        Version,
        lang::{MultiLangVersionContainer, VersionLanguage},
    },
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
    pub stats: Vec<CatFormStats>,
    /// Animation data for each form.
    pub anims: Vec<CatFormAnimData>,
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
    pub unitbuy: UnitBuy,
    /// Data from `unitexp.csv`.
    pub unitexp: XPCostScale,
    /// Data from `unitlevel.csv`.
    pub unitlevel: UnitLevelRaw,
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
    /// No data in `unitlevel.csv`.
    UnitLevelNotFound,
}

impl Cat {
    /// Get cat from wiki id.
    pub fn from_wiki_id<T: MultiLangVersionContainer>(
        wiki_id: u32,
        version_cont: &T,
    ) -> Result<Self, CatDataError> {
        type E = CatDataError;
        let id = wiki_id;

        let unitbuy = version_cont
            .lang_default()
            .get_cached_file::<UnitBuyContainer>();
        let unitbuy = UnitBuy::from_unitbuy(unitbuy.get_unit(id).ok_or(E::UnitBuyNotFound)?);

        let unitexp = XPCostScale::from_id(id);

        let unitlevel = version_cont
            .lang_default()
            .get_cached_file::<UnitLevelContainer>();
        let unitlevel = unitlevel.get_unit(id).ok_or(E::UnitLevelNotFound)?.clone();

        let is_summon = unitbuy.misc.is_summon();
        let has_true = unitbuy.true_evol.is_some();
        let has_ultra = unitbuy.ultra_evol.is_some();
        let egg_data = &unitbuy.misc.egg_info;

        let amt_forms = Self::get_amt_forms(id, is_summon, has_true, has_ultra);
        let forms = Self::get_forms(id, version_cont, amt_forms, egg_data);

        Ok(Self {
            id,
            forms,
            unitbuy,
            unitexp,
            unitlevel,
        })
    }

    fn get_amt_forms(id: u32, is_summon: bool, has_true: bool, has_ultra: bool) -> usize {
        match id {
            339 | 673 => 1,
            // iron wall, cheetah
            _ if is_summon => 1,
            _ => 2 + usize::from(has_true) + usize::from(has_ultra),
        }
    }

    fn get_forms<T: MultiLangVersionContainer>(
        id: u32,
        version_cont: &T,
        amt_forms: usize,
        egg_data: &AncientEggInfo,
    ) -> CatForms {
        let stats = Self::get_stats(id, version_cont.lang_default()).collect::<Vec<_>>();

        let get = |ver| get_anims(id, ver, amt_forms, egg_data);
        let anims = match get(version_cont.get_lang(VersionLanguage::EN)) {
            Ok(anims) => anims,
            Err(_) => get(version_cont.get_lang(VersionLanguage::JP)).unwrap(),
            // unwrap is probably a bad idea here
        };

        assert!(stats.len() >= amt_forms);
        assert!(anims.len() >= amt_forms);

        CatForms {
            amt_forms,
            stats,
            anims,
        }
    }

    /// Get stats for each form.
    pub fn get_stats(wiki_id: u32, version: &Version) -> impl Iterator<Item = CatFormStats> {
        // get_stats(wiki_id + 1, version)
        let abs_id = wiki_id + 1;
        let file_name = format!("unit{abs_id:03}.csv");
        let combined_iter = read_data_file(&file_name, version);
        combined_iter.map(|combined| CatFormStats::from_combined(&combined))
    }
}

impl Cat {
    /// Does the cat have a true form?
    pub fn has_true_form(&self) -> bool {
        self.forms.amt_forms > 2
    }

    /// Does the cat have an ultra form?
    pub fn has_ultra_form(&self) -> bool {
        self.forms.amt_forms > 3
    }
    // are these even needed?
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    #[ignore]
    fn test_all() {
        type E = CatDataError;
        for id in 0..u32::MAX {
            if matches!(id, (740..=745) | 788 | 810) {
                continue;
            }
            match Cat::from_wiki_id(id, &TEST_CONFIG.version) {
                Ok(_) => (),
                Err(E::UnitBuyNotFound | E::UnitLevelNotFound) => break,
            }
        }
    }
}
