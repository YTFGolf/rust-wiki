//! Deals with enemy encounters.

use crate::data::stage::raw::stage_data::StageData;

/// Does the stage contain the given enemy.
pub fn stage_contains_enemy(abs_enemy_id: u32, stage: &StageData) -> bool {
    stage
        .stage_csv_data
        .enemies
        .iter()
        .any(|e| e.num == abs_enemy_id)
}

/// Filter an iterator over stages by whether it contains the absolute enemy id.
pub fn filter_encounters<'a: 'b, 'b, I>(
    abs_enemy_id: u32,
    encounters: I,
) -> impl Iterator<Item = &'b StageData<'a>>
where
    I: Iterator<Item = &'b StageData<'a>>,
{
    encounters.filter(move |stage_data| stage_contains_enemy(abs_enemy_id, &stage_data))
}

/*
Due to how the encounters section is constantly evolving, this module cannot be
tested any other way than empirically.
*/
