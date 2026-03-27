//! Deals with unit animation data.

use super::unitbuy::AncientEggInfo;
use crate::game_data::{
    unit::anim::{Anim, AnimDataError, get_maanim_data},
    version::Version,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Data about a unit form's animations.
pub struct CatFormAnimData {
    /// Attack animation.
    pub attack: Anim,
}

fn get_anim_data(path: &str, version: &Version) -> Result<CatFormAnimData, AnimDataError> {
    Ok(CatFormAnimData {
        attack: get_maanim_data(path, version)?,
    })
}

/// Get unit animations.
pub fn get_cat_anims(
    wiki_id: u32,
    version: &Version,
    amt_forms: usize,
    egg_data: &AncientEggInfo,
) -> Result<Vec<CatFormAnimData>, (AnimDataError, usize)> {
    let (form1, form2) = match egg_data {
        AncientEggInfo::None => (
            format!("{wiki_id:03}_f02.maanim"),
            format!("{wiki_id:03}_c02.maanim"),
            // I think 02 means the attack animation
        ),
        AncientEggInfo::Egg { normal, evolved } => (
            format!("{normal:03}_m02.maanim"),
            format!("{evolved:03}_m02.maanim"),
        ),
    };

    let mut anims = vec![get_anim_data(&form1, version).map_err(|e| (e, 1))?];
    if amt_forms > 1 {
        anims.push(get_anim_data(&form2, version).map_err(|e| (e, 2))?);
    }
    if amt_forms > 2 {
        let tf = format!("{wiki_id:03}_s02.maanim");
        anims.push(get_anim_data(&tf, version).map_err(|e| (e, 3))?);
    }
    if amt_forms > 3 {
        let uf = format!("{wiki_id:03}_u02.maanim");
        anims.push(get_anim_data(&uf, version).map_err(|e| (e, 4))?);
    }

    Ok(anims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        TEST_CONFIG,
        game_data::cat::{parsed::unitbuy::UnitBuy, raw::unitbuy::UnitBuyContainer},
    };

    /// Egg data, amount of forms.
    fn get_egg_data(id: u32, version: &Version) -> (AncientEggInfo, usize) {
        let unitbuy = version.get_cached_file::<UnitBuyContainer>();
        let unitbuy = UnitBuy::from_unitbuy(unitbuy.get_unit(id).unwrap());

        let has_true = unitbuy.true_evol.is_some();
        let has_ultra = unitbuy.ultra_evol.is_some();
        let egg_data = unitbuy.misc.egg_info;

        let amt_forms = 2 + usize::from(has_true) + usize::from(has_ultra);
        // TODO use methods from `Cat` to achieve this.
        (egg_data, amt_forms)
    }

    fn try_all_anims(
        id: u32,
        version: &Version,
    ) -> Result<Vec<CatFormAnimData>, (AnimDataError, usize)> {
        let (egg, amt) = get_egg_data(id, version);
        get_cat_anims(id, version, amt, &egg)
    }

    #[track_caller]
    fn get_all_anims(id: u32, version: &Version) -> Vec<CatFormAnimData> {
        try_all_anims(id, version).unwrap()
    }

    fn anim(length: u16) -> CatFormAnimData {
        CatFormAnimData {
            attack: Anim { length },
        }
    }

    #[test]
    fn basic() {
        let version = TEST_CONFIG.version.fallback();

        let cat = get_all_anims(0, version);
        let ans = [anim(18), anim(16), anim(16)];

        assert_eq!(&cat, &ans);
    }

    #[test]
    fn basic2() {
        let version = TEST_CONFIG.version.fallback();

        let tank = get_all_anims(1, version);
        let ans = [anim(16), anim(16), anim(16)];

        assert_eq!(&tank, &ans);
    }

    #[test]
    fn basic3() {
        let version = TEST_CONFIG.version.fallback();

        let titan = get_all_anims(8, version);
        let ans = [anim(26), anim(32), anim(32)];

        assert_eq!(&titan, &ans);
    }

    #[test]
    fn basic4() {
        let version = TEST_CONFIG.version.fallback();

        let actress = get_all_anims(9, version);
        let ans = [anim(12), anim(16), anim(16)];

        assert_eq!(&actress, &ans);
    }

    #[test]
    fn backswing_multihit() {
        let version = TEST_CONFIG.version.fallback();

        let bahamut = get_all_anims(25, version);
        let ans = [anim(151), anim(151), anim(93)];

        assert_eq!(&bahamut, &ans);
    }

    #[test]
    fn long_foreswing() {
        let version = TEST_CONFIG.version.fallback();

        let cancan = get_all_anims(32, version);
        let ans = [anim(46), anim(46), anim(46)];

        assert_eq!(&cancan, &ans);
    }

    #[test]
    fn repeated() {
        let version = TEST_CONFIG.version.fallback();

        let dom = get_all_anims(13, version);
        let ans = [anim(45), anim(45), anim(12)];

        assert_eq!(&dom, &ans);
    }

    #[test]
    fn multihit() {
        let version = TEST_CONFIG.version.fallback();

        let delinquent = get_all_anims(31, version);
        let ans = [anim(73), anim(66), anim(66)];

        assert_eq!(&delinquent, &ans);
    }

    #[test]
    fn dio() {
        let version = TEST_CONFIG.version.fallback();

        let dio = get_all_anims(177, version);
        let ans = [anim(146), anim(146), anim(165), anim(175)];

        assert_eq!(&dio, &ans);
    }

    #[test]
    fn metal() {
        let version = TEST_CONFIG.version.fallback();

        let metal = get_all_anims(200, version);
        let ans = [anim(18), anim(16)];

        assert_eq!(&metal, &ans);
    }

    #[test]
    fn full_backswing() {
        let version = TEST_CONFIG.version.fallback();

        let dasli = get_all_anims(543, version);
        let ans = [anim(171), anim(174)];

        assert_eq!(&dasli, &ans);
    }

    #[test]
    fn backswing2() {
        let version = TEST_CONFIG.version.fallback();

        let cat_modoki = get_all_anims(626, version);
        let ans = [anim(10), anim(10)];

        assert_eq!(&cat_modoki, &ans);
    }

    #[test]
    fn partial_backswing() {
        let version = TEST_CONFIG.version.fallback();

        let sfeline = get_all_anims(643, version);
        let ans = [anim(31), anim(31), anim(74)];

        assert_eq!(&sfeline, &ans);
    }

    #[test]
    fn egg() {
        let version = TEST_CONFIG.version.fallback();

        let courier = get_all_anims(658, version);
        let ans = [anim(76), anim(76), anim(61)];

        assert_eq!(&courier, &ans);
    }

    #[test]
    fn kr_exclusive() {
        let version = TEST_CONFIG.version.fallback();

        let crew12 = get_all_anims(182, version);
        let ans = [anim(111), anim(111)];

        assert_eq!(&crew12, &ans);
    }
}
