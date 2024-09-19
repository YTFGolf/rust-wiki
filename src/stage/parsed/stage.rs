//! Represents a full stage.
/*
Meta

/// Data that always exists.
// definite data (DefiniteStageData)
base_id: i32,
width: u32,
/// Note: if the stage has an animated base this is not the correct base hp.
base_hp: u32,
max_enemies: u32,
is_no_continues: bool,
anim_base_id: Option<std::num::NonZeroU32>,
time_limit: Option<std::num::NonZeroU32>,
is_base_indestructible: bool
continue_data: Option<chance, map, stages()>
background_id: u32,
enemies: Vec<StageEnemy>,

/// Data related to maps and stage rewards.
// almost always exists (Option<MapAndRewardStageData>)
energy: Option<u32>
xp: Option<u32>
max_clears: Option<std::num::NonZeroU32>
cooldown: Option<std::num::NonZeroU32>
star_mask: Option<u32>
rewards (StageDataCSV): Option<new struct>

/// Data about
crown_data: Option<custom struct>
restrictions: Option<Vec<<custom struct>>> with `None`s and labels star difficulty.
*/
