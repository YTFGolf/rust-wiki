//! Deals with unit animation data.

use super::unitbuy::AncientEggInfo;
use crate::data::version::Version;
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

#[derive(Debug)]
/// Data about a unit form's animations.
pub struct AnimData {
    length: usize, // right now all that's needed is the length of the animation
}
impl AnimData {
    /// Get length of unit's animations.
    pub fn len(&self) -> usize {
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
        anims.push(get_anim_data(&form2, version).map_err(|e| (e, 2))?)
    }
    if amt_forms > 2 {
        let tf = format!("{wiki_id:03}_s02.maanim");
        anims.push(get_anim_data(&tf, version).map_err(|e| (e, 3))?)
    }
    if amt_forms > 3 {
        let uf = format!("{wiki_id:03}_u02.maanim");
        anims.push(get_anim_data(&uf, version).map_err(|e| (e, 4))?)
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
            };
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
        data::cat::{parsed::unitbuy::UnitBuyData, raw::unitbuy::UnitBuyContainer},
    };

    fn get_egg_data(id: u32, version: &Version) -> (AncientEggInfo, usize) {
        let unitbuy = version.get_cached_file::<UnitBuyContainer>();
        let unitbuy = UnitBuyData::from_unitbuy(unitbuy.get_unit(id).unwrap());

        let has_true = unitbuy.true_evol.is_some();
        let has_ultra = unitbuy.ultra_evol.is_some();
        let egg_data = unitbuy.misc.egg_info;

        let amt_forms = 2 + has_true as usize + has_ultra as usize;
        (egg_data, amt_forms)
    }

    #[test]
    #[ignore]
    fn test_units() {
        let version = TEST_CONFIG.version.current_version();

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
            let (egg, amt) = get_egg_data(id, version);
            println!("{name} ({id}) = {:#?}\n", get_anims(id, version, amt, &egg));
        }
        todo!()
    }
}
