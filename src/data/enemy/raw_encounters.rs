//! Deals with enemy encounters.

use crate::data::{stage::raw::stage_data::StageData, version::Version};
use regex::Regex;

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
    let stage_file_re = Regex::new(r"^stage.*?\d{2}\.csv$").unwrap();
    let dir = &version.get_file_path("DataLocal");

    let files = std::fs::read_dir(dir).unwrap();
    let encounters = files.filter_map(|f| {
        let file_name = f.unwrap().file_name().into_string().unwrap();
        if !stage_file_re.is_match(&file_name) {
            return None;
        };

        let stage = StageData::new(&file_name, version).unwrap();
        stage_contains_enemy(abs_enemy_id, &stage).then_some(stage)
    });

    encounters.collect()
}

/*
Due to how the encounters section is constantly evolving, `get_encounters`
cannot be tested any other way than empirically.
*/
