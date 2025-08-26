//! Deals with unit level-up scaling.

use crate::game_data::version::version_data::CacheableVersionData;
use std::path::Path;

#[derive(Debug, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
/// Level-up scale multiplier per 10 levels.
///
/// All values are multiplied by 100 to avoid using floats.
pub struct UnitLevelRaw {
    /// Level scale multiplier up to level 10.
    pub until_10: u8,
    /// Level scale multiplier up to level 20.
    pub until_20: u8,
    /// Level scale multiplier up to level 30.
    pub until_30: u8,
    /// Level scale multiplier up to level 40.
    pub until_40: u8,
    /// Level scale multiplier up to level 50.
    pub until_50: u8,
    /// Level scale multiplier up to level 60.
    pub until_60: u8,
    /// Level scale multiplier up to level 70.
    pub until_70: u8,
    /// Level scale multiplier up to level 80.
    pub until_80: u8,
    /// Level scale multiplier up to level 90.
    pub until_90: u8,
    /// Level scale multiplier up to level 100.
    pub until_100: u8,
    /// Level scale multiplier up to level 110.
    pub until_110: u8,
    /// Level scale multiplier up to level 120.
    pub until_120: u8,
    /// Level scale multiplier up to level 130.
    pub until_130: u8,
    /// Level scale multiplier up to level 140.
    pub until_140: u8,
    /// Level scale multiplier up to level 150.
    pub until_150: u8,
    /// Level scale multiplier up to level 160.
    pub until_160: u8,
    /// Level scale multiplier up to level 170.
    pub until_170: u8,
    /// Level scale multiplier up to level 180.
    pub until_180: u8,
    /// Level scale multiplier up to level 190.
    pub until_190: u8,
    /// Level scale multiplier up to level 200.
    pub until_200: u8,
}

impl UnitLevelRaw {
    /// Iterate through each level scale multiplier.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = u8> {
        [
            self.until_10,
            self.until_20,
            self.until_30,
            self.until_40,
            self.until_50,
            self.until_60,
            self.until_70,
            self.until_80,
            self.until_90,
            self.until_100,
            self.until_110,
            self.until_120,
            self.until_130,
            self.until_140,
            self.until_150,
            self.until_160,
            self.until_170,
            self.until_180,
            self.until_190,
            self.until_200,
        ]
        .into_iter()
    }
}

impl UnitLevelRaw {
    fn up_to_10(level: u8, until: u8) -> f64 {
        f64::from(level) * f64::from(until) / 100.0
    }

    fn get_stat_multiplier_at_level(&self, mut level: u8) -> f64 {
        assert!(level > 0);
        let mut total_multiplier = 1.0;

        if level <= 10 {
            return total_multiplier + Self::up_to_10(level - 1, self.until_10);
        }
        total_multiplier += Self::up_to_10(9, self.until_10);
        level -= 10;
        // needs to do - 1 for both calls to `up_to_10`, so can't use macro

        /// The rest is just the same code so use macro.
        macro_rules! level_split {
            ($input_name:expr) => {
                if level <= 10 {
                    return total_multiplier + Self::up_to_10(level, $input_name);
                }
                total_multiplier += Self::up_to_10(10, $input_name);
                level -= 10;
            };
        }

        // should probably refactor this to use `.iter()`
        level_split!(self.until_20);
        level_split!(self.until_30);
        level_split!(self.until_40);
        level_split!(self.until_50);
        level_split!(self.until_60);
        level_split!(self.until_70);
        level_split!(self.until_80);
        level_split!(self.until_90);
        level_split!(self.until_100);
        level_split!(self.until_110);
        level_split!(self.until_120);
        level_split!(self.until_130);
        level_split!(self.until_140);
        level_split!(self.until_150);
        level_split!(self.until_160);
        level_split!(self.until_170);
        level_split!(self.until_180);
        level_split!(self.until_190);
        level_split!(self.until_200);

        unreachable!("Reached {total_multiplier} with {level} levels remaining");
    }

    /// Raw, pre-treasure stat at level.
    pub fn get_raw_stat_at_level(&self, initial: u32, level: u8) -> u32 {
        let multiplier = self.get_stat_multiplier_at_level(level);
        let n = multiplier * f64::from(initial);
        n.round() as u32
    }

    /// Includes treasure bonuses.
    pub fn get_stat_at_level(&self, initial: u32, level: u8) -> u32 {
        self.get_raw_stat_at_level(initial, level) * 25 / 10
    }
}

fn get_unitlevel(path: &Path) -> Vec<UnitLevelRaw> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path.join("DataLocal/unitlevel.csv"))
        .unwrap();

    rdr.byte_records()
        .map(|record| {
            let result = record.unwrap();
            assert_eq!(result.len(), std::mem::size_of::<UnitLevelRaw>());
            // make sure that the struct definition is up-to-date
            let unit: UnitLevelRaw = result.deserialize(None).unwrap();
            unit
        })
        .collect()
}

#[derive(Debug)]
/// Container for [`UnitLevelRaw`] data.
pub struct UnitLevelContainer {
    units: Vec<UnitLevelRaw>,
}
impl UnitLevelContainer {
    /// Get [`UnitLevelRaw`] line for a unit.
    pub fn get_unit(&self, id: u32) -> Option<&UnitLevelRaw> {
        self.units.get(id as usize)
    }
}
impl CacheableVersionData for UnitLevelContainer {
    fn init_data(path: &Path) -> Self {
        Self {
            units: get_unitlevel(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TEST_CONFIG, game_data::version::Version};

    fn get_unitlevel(id: u32, version: &Version) -> &UnitLevelRaw {
        let unitlevel = version.get_cached_file::<UnitLevelContainer>();
        unitlevel.get_unit(id).unwrap()
    }

    #[test]
    fn standard_1() {
        let mohawk = get_unitlevel(0, TEST_CONFIG.version.current_version());

        let init_hp = 200;
        let init_ap = 8;

        assert_eq!(mohawk.get_raw_stat_at_level(init_hp, 1), init_hp);
        assert_eq!(mohawk.get_raw_stat_at_level(init_ap, 1), init_ap);
        assert_eq!(mohawk.get_stat_at_level(init_hp, 1), init_hp * 25 / 10);
        assert_eq!(mohawk.get_stat_at_level(init_ap, 1), init_ap * 25 / 10);
    }

    #[test]
    fn basic_stat_at_level() {
        let mohawk = get_unitlevel(0, TEST_CONFIG.version.current_version());

        let init_hp = 200;
        let init_ap = 8;

        assert_eq!(mohawk.get_stat_at_level(init_hp, 30), init_hp * 17);
        assert_eq!(mohawk.get_stat_at_level(init_ap, 30), 135);
        // init_ap not divisible by 5 so ends up not being * 17
    }

    #[test]
    fn standard_stat_at_level() {
        let dasli = get_unitlevel(543, TEST_CONFIG.version.current_version());

        let init_hp = 4_600;
        let init_ap = 1_000;

        assert_eq!(dasli.get_stat_at_level(init_hp, 30), init_hp * 17);
        assert_eq!(dasli.get_stat_at_level(init_hp, 30), 78_200);
        assert_eq!(dasli.get_stat_at_level(init_ap, 30), init_ap * 17);
        assert_eq!(dasli.get_stat_at_level(init_ap, 30), 17_000);
    }

    #[test]
    fn standard_uber_60_and_max() {
        let dio = get_unitlevel(177, TEST_CONFIG.version.current_version());

        let init_hp = 14_800;
        let init_ap = 5_600;

        assert_eq!(dio.get_stat_at_level(init_hp, 60), init_hp * 32);
        assert_eq!(dio.get_stat_at_level(init_hp, 60), 473_600);
        assert_eq!(dio.get_stat_at_level(init_ap, 60), init_ap * 32);
        assert_eq!(dio.get_stat_at_level(init_ap, 60), 179_200);

        assert_eq!(dio.get_stat_at_level(init_hp, 60 + 70), 640_100);
        assert_eq!(dio.get_stat_at_level(init_ap, 60 + 70), 242_200);
    }

    #[test]
    fn metal() {
        let metal = get_unitlevel(200, TEST_CONFIG.version.current_version());

        let init_hp = 1;
        let init_ap = 8;

        assert_eq!(metal.get_stat_at_level(init_hp, 20), 12);
        assert_eq!(metal.get_stat_at_level(init_ap, 20), 95);
        assert_eq!(metal.get_stat_at_level(init_hp, 19), 12);
        assert_eq!(metal.get_stat_at_level(init_ap, 19), 92);
        assert_eq!(metal.get_stat_at_level(init_hp, 18), 10);
        assert_eq!(metal.get_stat_at_level(init_ap, 18), 87);
    }

    #[test]
    fn bahamut() {
        let bahamut = get_unitlevel(25, TEST_CONFIG.version.current_version());

        let init_hp = 1_500;
        let init_ap = 5_000 + 200 + 300;

        assert_eq!(bahamut.get_stat_at_level(init_hp, 30), 25_500);
        assert_eq!(bahamut.get_stat_at_level(init_ap, 30), 93_500);
        assert_eq!(bahamut.get_stat_at_level(init_hp, 50), 33_000);
        assert_eq!(bahamut.get_stat_at_level(init_ap, 50), 121_000);
    }

    #[test]
    fn gacha() {
        let gacha = get_unitlevel(558, TEST_CONFIG.version.current_version());

        let init_hp = 1_500;
        let init_ap = 1;

        assert_eq!(gacha.get_stat_at_level(init_hp, 20), 18_000);
        assert_eq!(gacha.get_stat_at_level(init_ap, 20), 12);
        assert_eq!(gacha.get_stat_at_level(init_hp, 21), 20_250);
        assert_eq!(gacha.get_stat_at_level(init_ap, 21), 12);
        assert_eq!(gacha.get_stat_at_level(init_hp, 22), 22_500);
        assert_eq!(gacha.get_stat_at_level(init_ap, 22), 15);
        assert_eq!(gacha.get_stat_at_level(init_hp, 50), 153_000);
        assert_eq!(gacha.get_stat_at_level(init_ap, 50), 102);
    }

    #[test]
    fn ramen() {
        let ramen = get_unitlevel(148, TEST_CONFIG.version.current_version());

        let init_hp = 1_050;
        // level-up page only talks about hp

        assert_eq!(ramen.get_stat_at_level(init_hp, 25), 15_225);
        assert_eq!(ramen.get_stat_at_level(init_hp, 75), 40_162);
        assert_eq!(ramen.get_stat_at_level(init_hp, 95), 44_757);
    }
}
