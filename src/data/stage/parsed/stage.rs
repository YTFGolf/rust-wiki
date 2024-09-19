//! Represents a full stage.

use super::stage_enemy::StageEnemy;
use crate::{
    data::map::map_data::csv_types::{ScoreRewardsCSV, TreasureCSV, TreasureType},
    data::stage::{
        stage_data::StageData,
        stage_metadata::StageMeta,
        stage_option::{
            charagroups::{CharaGroup, CHARAGROUP},
            StageOptionCSV,
        },
    },
};
use std::num::NonZeroU32;

#[derive(Debug)]
/// Rewards for the stage.
pub struct StageRewards {
    /// Modifier for the treasure drop.
    pub treasure_type: TreasureType,
    /// Raw treasure drop data.
    pub treasure_drop: Vec<TreasureCSV>,
    /// Raw score rewards data.
    pub score_rewards: Vec<ScoreRewardsCSV>,
}

#[derive(Debug)]
/// Possible continuation stages.
pub struct ContinueStages {
    /// Chance of continuing.
    pub chance: u32,
    /// EX stage map id.
    pub map_id: u32,
    /// `(min, max)` pair of stage ids.
    pub stage_ids: (u32, u32),
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
pub struct CrownData {
    /// Max crown difficulty.
    pub max_difficulty: std::num::NonZeroU8,
    /// 2-crown magnification.
    pub crown_2: Option<NonZeroU32>,
    /// 3-crown magnification.
    pub crown_3: Option<NonZeroU32>,
    /// 4-crown magnification.
    pub crown_4: Option<NonZeroU32>,
}

#[derive(Debug)]
/// Crowns that restriction applies to.
pub enum RestrictionCrowns {
    /// All crown difficulties.
    All,
    /// Only one difficulty.
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
pub struct Restriction {
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
impl From<&StageOptionCSV> for Restriction {
    fn from(value: &StageOptionCSV) -> Self {
        let charagroup = NonZeroU32::new(value.charagroup)
            .map(|value| CHARAGROUP.get_charagroup(value.into()).unwrap());

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

/// Full stage struct.
pub struct Stage {
    /// Stage's metadata.
    pub meta: StageMeta,

    // Data that always exists.
    /// ID of enemy base (if [anim_base_id][Self::anim_base_id] exists then that
    /// overrides this flag).
    pub base_id: i32,
    /// Does the stage have no continues.
    pub is_no_continues: bool,
    /// Data about possible continuation stages.
    pub continue_data: Option<ContinueStages>,
    /// Stage width.
    pub width: u32,
    /// Base's HP (if [anim_base_id][Self::anim_base_id] exists then base HP is
    /// the HP of that enemy, not this field).
    pub base_hp: u32,
    /// Max enemies that can spawn.
    pub max_enemies: u32,
    /// ID of animated base.
    pub anim_base_id: Option<NonZeroU32>,
    /// Time limit of stage.
    pub time_limit: Option<NonZeroU32>,
    /// Is base indestructible until boss dies.
    pub is_base_indestructible: bool,
    /// ID of the stage's background.
    pub background_id: u32,
    /// List of enemies in stage.
    pub enemies: Vec<StageEnemy>,

    // Data related to maps and stage rewards. Almost always exists, except for
    // like Labyrinth.
    /// Energy cost of stage.
    // TODO figure out catamin stuff.
    pub energy: Option<u32>,
    /// Base XP reward of stage.
    pub xp: Option<u32>,
    /// Rewards available.
    /// Note: if the stage has an animated base this is not the correct base hp.
    pub rewards: Option<StageRewards>,

    /// Max clears before stage disappears.
    pub max_clears: Option<NonZeroU32>,
    /// Gauntlet cooldown.
    pub cooldown: Option<NonZeroU32>,
    /// Binary mask of the star difficulty.
    pub star_mask: Option<u16>,
    /// Crown difficulties of stage.
    pub crown_data: Option<CrownData>,

    /// Stage's restrictions.
    pub restrictions: Option<Vec<Restriction>>,
}
impl From<StageData> for Stage {
    fn from(data: StageData) -> Self {
        let map_stage_data = data.get_map_stage_data();
        let map_option_data = data.get_map_option_data();

        let restrictions: Option<Vec<Restriction>>;
        if let Some(data) = data.get_stage_option_data() {
            restrictions = Some(data.into_iter().map(|r| r.into()).collect());
        } else {
            restrictions = None;
        }

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

        let energy: Option<u32>;
        let xp: Option<u32>;
        let rewards: Option<StageRewards>;
        if let Some(data) = map_stage_data {
            energy = Some(data.fixed_data.energy);
            xp = Some(data.fixed_data.xp);
            rewards = Some(StageRewards {
                treasure_type: data.treasure_type,
                treasure_drop: data.treasure_drop,
                score_rewards: data.score_rewards,
            });
        } else {
            energy = None;
            xp = None;
            rewards = None;
        }

        let max_clears: Option<NonZeroU32>;
        let cooldown: Option<NonZeroU32>;
        let star_mask: Option<u16>;
        let crown_data: Option<CrownData>;

        if let Some(data) = map_option_data {
            max_clears = NonZeroU32::new(data.max_clears);
            cooldown = NonZeroU32::new(data.cooldown);
            star_mask = Some(data.star_mask);
            crown_data = Some(CrownData {
                max_difficulty: data.max_difficulty,
                crown_2: NonZeroU32::new(data.crown_2),
                crown_3: NonZeroU32::new(data.crown_3),
                crown_4: NonZeroU32::new(data.crown_4),
            })
        } else {
            max_clears = None;
            cooldown = None;
            star_mask = None;
            crown_data = None;
        }

        Self {
            meta,
            base_id,
            is_no_continues,
            continue_data,
            width,
            base_hp,
            max_enemies,
            anim_base_id,
            time_limit,
            is_base_indestructible,
            background_id,
            enemies,
            energy,
            xp,
            rewards,
            max_clears,
            cooldown,
            star_mask,
            crown_data,
            restrictions,
        }
    }
}
fn u8_to_bool(n: u8) -> bool {
    match n {
        0 => false,
        1 => true,
        n => panic!("Value {n} is not a valid boolean number!"),
    }
}

impl Stage {
    /// Create a new stage object from `selector`.
    pub fn new(selector: &str) -> Option<Self> {
        Some(StageData::new(selector)?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_handler::{get_file_location, FileLocation::GameData};
    use regex::Regex;

    #[test]
    #[ignore]
    fn get_all() {
        let stage_file_re = Regex::new(r"^stage.*?\d{2}\.csv$").unwrap();
        for f in std::fs::read_dir(get_file_location(GameData).join("DataLocal")).unwrap() {
            let file_name = f.unwrap().file_name().into_string().unwrap();
            if !stage_file_re.is_match(&file_name) {
                continue;
            };
            let _stage = Stage::new(&file_name).unwrap();
        }
    }
}