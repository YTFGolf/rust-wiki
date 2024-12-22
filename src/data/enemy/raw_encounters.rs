//! Deals with enemy encounters.

use crate::data::{
    stage::{get_stages, raw::stage_data::StageData},
    version::Version,
};

/// Does the stage contain the given enemy.
pub fn stage_contains_enemy(abs_enemy_id: u32, stage: &StageData) -> bool {
    stage
        .stage_csv_data
        .enemies
        .iter()
        .any(|e| e.num == abs_enemy_id)
}

/// Get the enemy's encounters.
///
/// `abs_enemy_id` means num in game files, i.e. Doge = 2.
pub fn get_encounters(abs_enemy_id: u32, version: &Version) -> Vec<StageData<'_>> {
    let encounters =
        get_stages(version).filter(|stage_data| stage_contains_enemy(abs_enemy_id, &stage_data));

    encounters.collect()
}

/*
Due to how the encounters section is constantly evolving, `get_encounters`
cannot be tested any other way than empirically.
*/
