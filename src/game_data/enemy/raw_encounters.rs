//! Deals with enemy encounters.

use crate::game_data::stage::raw::stage_data::StageData;

/// Does the stage contain the given enemy. Useful for filter operations.
///
/// ```rust,no_run
/// # use rust_wiki::game_data::enemy::raw_encounters::stage_contains_enemy;
/// # use rust_wiki::game_data::stage::{raw::stage_data::StageData, stage_util::get_stage_files};
/// # use rust_wiki::game_data::version::Version;
/// # use rust_wiki::SLang;
/// # let version = Version::new("~", SLang::EN, Some("1.0".into()));
/// let abs_enemy_id = 2;
/// let all_stages = get_stage_files(&version)
///     .map(|file_name| StageData::from_file_name(&file_name, &version).unwrap())
///     .collect::<Vec<_>>();
///
/// let encounters_iter = all_stages
///     .iter()
///     .filter(|s| stage_contains_enemy(abs_enemy_id, s));
/// let encounters = encounters_iter.collect::<Vec<_>>();
/// ```
pub fn stage_contains_enemy(abs_enemy_id: u32, stage: &StageData) -> bool {
    stage
        .stage_csv_data
        .enemies
        .iter()
        .any(|e| e.num == abs_enemy_id)
}
