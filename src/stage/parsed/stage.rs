//! Represents a full stage.
/*
Meta

/// Data that always exists.
// definite data
base_id: i32,
width: u32,
/// Note: if the stage has an animated base this is not the correct base hp.
base_hp: u32,
max_enemies: u32,
is_no_continues: bool,
anim_base_id: Option<std::num::NonZeroU32>,
time_limit: Option<std::num::NonZeroU32>,
is_base_indestructible: bool
continue_data: Option<ContinueStages>
background_id: u32,
enemies: Vec<StageEnemy>,

/// Data related to maps and stage rewards.
// almost always exists, except for like Labyrinth
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
use crate::{
    map::map_data::csv_types::{ScoreRewardsCSV, TreasureCSV, TreasureType},
    stage::stage_option::{
        charagroups::{CharaGroup, CHARAGROUP},
        StageOptionCSV,
    },
};

#[derive(Debug)]
/// Rewards for the stage.
struct StageRewards {
    /// Modifier for the treasure drop.
    pub treasure_type: TreasureType,
    /// Raw treasure drop data.
    pub treasure_drop: Vec<TreasureCSV>,
    /// Raw score rewards data.
    pub score_rewards: Vec<ScoreRewardsCSV>,
}

#[derive(Debug)]
/// Possible continuation stages.
struct ContinueStages {
    /// Chance of continuing.
    chance: u32,
    /// EX stage map id.
    map: u32,
    /// `(min, max)` pair of stage ids.
    stages: (u32, u32),
}

#[derive(Debug)]
/// Crown difficulty data.
struct CrownData {
    /// Max crown difficulty.
    pub max_difficulty: u8,
    /// 1-crown magnification.
    pub star_1: Option<std::num::NonZeroU32>,
    /// 2-crown magnification.
    pub star_2: Option<std::num::NonZeroU32>,
    /// 3-crown magnification.
    pub star_3: Option<std::num::NonZeroU32>,
    /// 4-crown magnification.
    pub star_4: Option<std::num::NonZeroU32>,
}

#[derive(Debug)]
/// Stage's restriction.
struct Restriction {
    /// Rarities allowed.
    pub rarity: Option<std::num::NonZeroU8>,
    /// Cat deploy limit.
    pub deploy_limit: Option<std::num::NonZeroU32>,
    /// Rows allowed.
    pub rows: Option<std::num::NonZeroU8>,
    /// Minimum cat cost.
    pub min_cost: Option<std::num::NonZeroU32>,
    /// Maximum cat cost.
    pub max_cost: Option<std::num::NonZeroU32>,
    /// Restricts you to either being unable to deploy specific units or only
    /// being able to deploy specific units.
    pub charagroup: Option<&'static CharaGroup>,
}
impl From<StageOptionCSV> for Restriction {
    fn from(value: StageOptionCSV) -> Self {
        let charagroup = match std::num::NonZeroU32::new(value.charagroup) {
            Some(value) => Some(CHARAGROUP.get_charagroup(value.into()).unwrap()),
            None => None,
        };

        Self {
            rarity: std::num::NonZeroU8::new(value.rarity),
            deploy_limit: std::num::NonZeroU32::new(value.deploy_limit),
            rows: std::num::NonZeroU8::new(value.rows),
            min_cost: std::num::NonZeroU32::new(value.min_cost),
            max_cost: std::num::NonZeroU32::new(value.max_cost),
            charagroup,
        }
    }
}
