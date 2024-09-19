//! Represents a full stage.

use std::num::NonZeroU32;

use super::stage_enemy::StageEnemy;
use crate::{
    map::map_data::csv_types::{ScoreRewardsCSV, TreasureCSV, TreasureType},
    stage::{
        stage_data::StageData,
        stage_metadata::StageMeta,
        stage_option::{
            charagroups::{CharaGroup, CHARAGROUP},
            StageOptionCSV,
        },
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
    map_id: u32,
    /// `(min, max)` pair of stage ids.
    stage_ids: (u32, u32),
}
impl ContinueStages {
    fn new(chance: u32, map_id: u32, stage_id_min: u32, stage_id_max: u32) -> Self {
        Self {
            chance,
            map_id,
            stage_ids: (stage_id_min, stage_id_max),
        }
    }
}

#[derive(Debug)]
/// Crown difficulty data.
struct CrownData {
    /// Max crown difficulty.
    pub max_difficulty: u8,
    /// 1-crown magnification.
    pub star_1: Option<NonZeroU32>,
    /// 2-crown magnification.
    pub star_2: Option<NonZeroU32>,
    /// 3-crown magnification.
    pub star_3: Option<NonZeroU32>,
    /// 4-crown magnification.
    pub star_4: Option<NonZeroU32>,
}

#[derive(Debug)]
pub enum RestrictionCrowns {
    All,
    One(std::num::NonZeroU8),
}
impl From<i8> for RestrictionCrowns {
    fn from(value: i8) -> Self {
        match std::num::NonZeroU8::new((value + 1) as u8) {
            None => Self::All,
            Some(value) => Self::One(value),
        }
    }
}

#[derive(Debug)]
/// Stage's restriction.
struct Restriction {
    /// Crown difficulties that the restrictions apply to.
    pub crowns_applied: RestrictionCrowns,
    /// Rarities allowed.
    pub rarity: Option<std::num::NonZeroU8>,
    /// Cat deploy limit.
    pub deploy_limit: Option<NonZeroU32>,
    /// Rows allowed.
    pub rows: Option<std::num::NonZeroU8>,
    /// Minimum cat cost.
    pub min_cost: Option<NonZeroU32>,
    /// Maximum cat cost.
    pub max_cost: Option<NonZeroU32>,
    /// Restricts you to either being unable to deploy specific units or only
    /// being able to deploy specific units.
    pub charagroup: Option<&'static CharaGroup>,
}
impl From<StageOptionCSV> for Restriction {
    fn from(value: StageOptionCSV) -> Self {
        let charagroup = match NonZeroU32::new(value.charagroup) {
            Some(value) => Some(CHARAGROUP.get_charagroup(value.into()).unwrap()),
            None => None,
        };

        Self {
            crowns_applied: value.stars.into(),
            rarity: std::num::NonZeroU8::new(value.rarity),
            deploy_limit: NonZeroU32::new(value.deploy_limit),
            rows: std::num::NonZeroU8::new(value.rows),
            min_cost: NonZeroU32::new(value.min_cost),
            max_cost: NonZeroU32::new(value.max_cost),
            charagroup,
        }
    }
}

struct Stage {
    meta: StageMeta,

    // Data that always exists.
    base_id: i32,
    is_no_continues: bool,
    continue_data: Option<ContinueStages>,
    width: u32,
    /// Note: if the stage has an animated base this is not the correct base hp.
    base_hp: u32,
    max_enemies: u32,
    anim_base_id: Option<NonZeroU32>,
    time_limit: Option<NonZeroU32>,
    is_base_indestructible: bool,
    background_id: u32,
    enemies: Vec<StageEnemy>,

    // Data related to maps and stage rewards. Almost always exists, except for
    // like Labyrinth.
    energy: Option<u32>,
    xp: Option<u32>,
    max_clears: Option<NonZeroU32>,
    cooldown: Option<NonZeroU32>,
    star_mask: Option<u32>,
    rewards: StageRewards,

    /// Data about
    crown_data: CrownData,
    restrictions: Vec<Restriction>,
}
impl From<StageData> for Stage {
    fn from(data: StageData) -> Self {
        let meta = data.meta;

        let base_id: i32 = data.stage_csv_data.header.base_id;
        let is_no_continues: bool = u8_to_bool(data.stage_csv_data.header.no_cont);
        let continue_data: Option<ContinueStages> = match data.stage_csv_data.header.cont_chance {
            0 => None,
            chance => Some(ContinueStages::new(
                chance,
                data.stage_csv_data.header.cont_map_id,
                data.stage_csv_data.header.cont_stage_id_min,
                data.stage_csv_data.header.cont_stage_id_max,
            )),
        };

        let max_enemies: u32 = data.stage_csv_data.line2.max_enemies;
        let width: u32 = data.stage_csv_data.line2.width;
        let base_hp: u32 = data.stage_csv_data.line2.base_hp;
        let anim_base_id: Option<NonZeroU32> =
            NonZeroU32::new(data.stage_csv_data.line2.anim_base_id);
        let time_limit: Option<NonZeroU32> = NonZeroU32::new(data.stage_csv_data.line2.time_limit);
        let is_base_indestructible: bool = u8_to_bool(data.stage_csv_data.line2.indestructible);
        let background_id: u32 = data.stage_csv_data.line2.background_id;
        let enemies: Vec<StageEnemy> = data
            .stage_csv_data
            .enemies
            .into_iter()
            .map(|e| e.into())
            .collect();

        todo!()
    }
}
fn u8_to_bool(n: u8) -> bool {
    match n {
        0 => false,
        1 => true,
        n => panic!("Value {n} is not a valid boolean number!"),
    }
}
