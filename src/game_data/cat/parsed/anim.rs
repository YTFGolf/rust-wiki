//! Deals with unit animation data.

use super::unitbuy::AncientEggInfo;
use crate::game_data::version::Version;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
/// Error when getting animation data.
pub enum AnimDataError {
    /// Specific form not found.
    FormNotFound,
    /// Animation is found but has no frames.
    EmptyAnimation,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Data about a unit form's animations.
pub struct AnimData {
    length: usize, // right now all that's needed is the length of the animation
}
impl AnimData {
    /// Get length of unit's animations.
    pub fn length(&self) -> usize {
        // not called `len` to avoid setting off clippy::len_without_is_empty
        self.length
    }
}

/// Get unit animations.
pub fn get_anims(
    wiki_id: u32,
    version: &Version,
    amt_forms: usize,
    egg_data: &AncientEggInfo,
) -> Result<Vec<AnimData>, (AnimDataError, usize)> {
    // needs to be tested with en first, then do jp if en doesn't work
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

fn get_anim_data(path: &str, version: &Version) -> Result<AnimData, AnimDataError> {
    use AnimDataError as E;
    const ANIM_LINE_LEN: usize = 4;
    let qualified = version.get_file_path("ImageDataLocal").join(path);
    let count = BufReader::new(File::open(&qualified).map_err(|_| E::FormNotFound)?)
        .lines()
        .filter_map(|line| {
            let line = line.as_ref().ok()?;
            let count = line.chars().filter(|c| *c == ',').count() + 1;
            // if has 4 items then has 3 commas
            if count != ANIM_LINE_LEN {
                return None;
            }
            let frame_no = line.split(',').next()?;
            frame_no.parse::<usize>().ok()
        })
        .max()
        .ok_or(E::EmptyAnimation)?;
    Ok(AnimData { length: count + 1 })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::TEST_CONFIG,
        game_data::cat::{parsed::unitbuy::UnitBuyData, raw::unitbuy::UnitBuyContainer},
    };

    /// Egg data, amount of forms.
    fn get_egg_data(id: u32, version: &Version) -> (AncientEggInfo, usize) {
        let unitbuy = version.get_cached_file::<UnitBuyContainer>();
        let unitbuy = UnitBuyData::from_unitbuy(unitbuy.get_unit(id).unwrap());

        let has_true = unitbuy.true_evol.is_some();
        let has_ultra = unitbuy.ultra_evol.is_some();
        let egg_data = unitbuy.misc.egg_info;

        let amt_forms = 2 + has_true as usize + has_ultra as usize;
        (egg_data, amt_forms)
    }

    fn get_all_anims(id: u32, version: &Version) -> Vec<AnimData> {
        let (egg, amt) = get_egg_data(id, version);
        get_anims(id, version, amt, &egg).unwrap()
    }

    fn anim(length: usize) -> AnimData {
        AnimData { length }
    }

    #[test]
    fn basic() {
        let version = TEST_CONFIG.version.jp();

        let cat = get_all_anims(0, version);
        let ans = [anim(18), anim(16), anim(16)];

        assert_eq!(&cat, &ans);
    }

    #[test]
    fn basic2() {
        let version = TEST_CONFIG.version.jp();

        let tank = get_all_anims(1, version);
        let ans = [anim(16), anim(16), anim(16)];

        assert_eq!(&tank, &ans);
    }

    #[test]
    fn basic3() {
        let version = TEST_CONFIG.version.jp();

        let titan = get_all_anims(8, version);
        let ans = [anim(26), anim(32), anim(32)];

        assert_eq!(&titan, &ans);
    }

    #[test]
    fn basic4() {
        let version = TEST_CONFIG.version.jp();

        let actress = get_all_anims(9, version);
        let ans = [anim(12), anim(16), anim(16)];

        assert_eq!(&actress, &ans);
    }

    #[test]
    fn backswing_multihit() {
        let version = TEST_CONFIG.version.jp();

        let bahamut = get_all_anims(25, version);
        let ans = [anim(151), anim(151), anim(93)];

        assert_eq!(&bahamut, &ans);
    }

    #[test]
    fn long_foreswing() {
        let version = TEST_CONFIG.version.jp();

        let cancan = get_all_anims(32, version);
        let ans = [anim(46), anim(46), anim(46)];

        assert_eq!(&cancan, &ans);
    }

    #[test]
    fn dio() {
        let version = TEST_CONFIG.version.jp();

        let dio = get_all_anims(177, version);
        let ans = [anim(146), anim(146), anim(165), anim(175)];

        assert_eq!(&dio, &ans);
    }

    #[test]
    fn metal() {
        let version = TEST_CONFIG.version.jp();

        let metal = get_all_anims(200, version);
        let ans = [anim(18), anim(16)];

        assert_eq!(&metal, &ans);
    }

    #[test]
    fn full_backswing() {
        let version = TEST_CONFIG.version.jp();

        let dasli = get_all_anims(543, version);
        let ans = [anim(171), anim(174)];

        assert_eq!(&dasli, &ans);
    }

    #[test]
    fn backswing2() {
        let version = TEST_CONFIG.version.jp();

        let cat_modoki = get_all_anims(626, version);
        let ans = [anim(10), anim(10)];

        assert_eq!(&cat_modoki, &ans);
    }

    #[test]
    fn partial_backswing() {
        let version = TEST_CONFIG.version.jp();

        let sfeline = get_all_anims(643, version);
        let ans = [anim(31), anim(31), anim(74)];

        assert_eq!(&sfeline, &ans);
    }

    #[test]
    fn egg() {
        let version = TEST_CONFIG.version.jp();

        let courier = get_all_anims(658, version);
        let ans = [anim(76), anim(76), anim(61)];

        assert_eq!(&courier, &ans);
    }
}
