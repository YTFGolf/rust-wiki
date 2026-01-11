use crate::game_data::{
    cat::raw::unitbuy::parse_unitbuy_error,
    version::{
        Version,
        version_data::{CacheableVersionData, CvdCreateError, CvdResult},
    },
};
use string_error::into_err;

/// Could reasonably either go above 65,535 or be multiplied to go above (e.g.
/// stats).
type Massive = u32;
/// Big number from 0 to 65,535.
type Big = u16;
/// 0-100.
type Percent = u8;
/// 0-256.
type Small = u8;
/// 0 or 1.
type Bool = u8;

#[derive(Debug, serde::Deserialize, Default)]
#[allow(missing_docs)]
pub struct EnemyCSV {
    pub hp: Massive,
    pub kb: Massive,
    pub spd: Big,
    pub dmg: Massive,
    pub tba: Big,
    pub range: Big,
    pub money_drop: Big,
    /// "Collision start". Always 0. Probably works the same as width.
    _uk7: Small,
    /// Unit width, always 320.
    _width: Big,
    _uk9: Small,

    // 10
    pub red: Bool,
    pub area: Bool,
    pub foreswing: Massive,
    pub floating: Bool,
    pub black: Bool,
    pub metal: Bool,
    pub traitless: Bool,
    pub angel: Bool,
    pub alien: Bool,
    pub zombie: Bool,

    // 20
    pub kb_chance: Percent,
    pub freeze_chance: Percent,
    pub freeze_duration: Big,
    pub slow_chance: Percent,
    pub slow_duration: Big,
    pub crit_chance: Percent,
    pub has_base_destroyer: Bool,
    pub wave_chance: Percent,
    pub wave_level: Small,
    pub weaken_chance: Percent,

    // 30
    pub weaken_duration: Big,
    pub weaken_multiplier: Percent,
    pub strengthen_hp: Percent,
    pub strengthen_multiplier: Big,
    pub survives_chance: Percent,
    pub ld_base: i16,
    pub ld_range: i16,
    pub immune_wave: Bool,
    pub has_wave_blocker: Bool,
    pub immune_kb: Bool,

    // 40
    pub immune_freeze: Bool,
    pub immune_slow: Bool,
    pub immune_weaken: Bool,
    /// -1 = infinite, 0 = none, rest is amount
    pub burrow_count: i8,
    pub burrow_dist_quad: Big,
    pub revive_count: i8,
    pub revive_delay: Big,
    pub revive_hp: Percent,
    pub witch: Bool,
    pub typeless: Bool,

    // 50
    /// "loop"
    _uk50: i8,
    _uk51: i8,
    pub kamikaze: Small,
    _uk53: i8,
    _uk54: i8,
    pub mhit_atk2: Massive,
    pub mhit_atk3: Massive,
    pub mhit_atk2_fswing: Big,
    pub mhit_atk3_fswing: Big,
    pub proc_on_hit1: Bool,

    // 60
    pub proc_on_hit2: Bool,
    pub proc_on_hit3: Bool,
    _uk62: Small,
    _uk63: Small,
    pub barrier_hp: Massive,
    pub warp_chance: Percent,
    pub warp_len: Big,
    pub warp_min_quad: i16,
    pub warp_max_quad: Big,
    pub starred_alien: Bool,

    // 70
    pub immune_warp: Bool,
    pub eva_angel: Bool,
    pub relic: Bool,
    pub curse_chance: Percent,
    pub curse_duration: Big,
    pub savage_blow_chance: Percent,
    pub savage_blow_percent: Big,
    pub dodge_chance: Percent,
    pub dodge_duration: Big,
    pub toxic_chance: Percent,

    // 80
    pub toxic_damage: Percent,
    pub surge_chance: Percent,
    pub surge_spawn_quad: Big,
    pub surge_range_quad: Big,
    pub surge_level: Small,
    pub immune_surge: Bool,
    pub shield_hp: Massive,
    pub shield_regen: Percent,
    pub death_surge_chance: Percent,
    pub death_surge_spawn_quad: Big,

    // 90
    pub death_surge_range_quad: Big,
    pub death_surge_level: Small,
    pub aku: Bool,
    pub colossus: Bool,

    rest: Vec<i32>,
}

#[derive(Debug)]
pub struct TUnitContainer {
    units: Vec<EnemyCSV>,
}
impl CacheableVersionData for TUnitContainer {
    fn create(version: &Version) -> CvdResult<Self> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(version.location().join("DataLocal/t_unit.csv"))
            .map_err(CvdCreateError::throw_from_err)?;

        let records: Result<Vec<EnemyCSV>, CvdCreateError<Self>> = rdr
            .byte_records()
            .map(|record| {
                let result = record.map_err(CvdCreateError::throw_from_err)?;
                let unit: EnemyCSV = match result.deserialize(None) {
                    Ok(u) => u,
                    Err(e) => {
                        let e2 = format!(
                            "Error when parsing record {result:?}: {e}. Item was {item:?}.",
                            item = parse_unitbuy_error(&e, &result)
                        );

                        return Err(CvdCreateError::throw(into_err(e2)));
                    }
                };

                Ok(unit)
            })
            .collect();

        Ok(Self { units: records? })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_CONFIG;

    #[test]
    fn test_file_reader() {
        let version = TEST_CONFIG.version.current_version();
        let t_unit = version.get_cached_file::<TUnitContainer>();
        let units = &t_unit.units;

        for unit in units {
            if unit.hp == 0 {
                // first two placeholders
                continue;
            }

            assert_eq!(unit._uk7, 0);
            assert_eq!(unit._width, 320);
            assert_eq!(unit._uk9, 0);
            // assert!(
            //     unit.rest.is_empty(),
            //     "Remaining fields not empty, found {:?}",
            //     unit.rest
            // );
            println!("{unit:?}");
        }
        panic!()
    }
}
