//! Represents a full stage.

use super::stage_enemy::StageEnemy;
use crate::game_data::{
    csv::FullCSVError,
    map::{
        cached::{map_option::MapOptionCSV, score_bonus::ScoreBonus, special_rules::SpecialRule},
        parsed::map::ResetType,
        raw::csv_types::{ScoreRewardsCSV, TreasureCSV, TreasureType},
    },
    meta::stage::{stage_id::StageID, stage_types::parse::parse_stage::parse_general_stage_id},
    stage::raw::{
        stage_data::{FromSelectorError, StageData},
        stage_option::{
            StageOptionCSV,
            charagroups::{CharaGroup, CharaGroups},
        },
    },
    version::Version,
};
use std::num::NonZeroU32;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
impl From<&MapOptionCSV> for CrownData {
    fn from(data: &MapOptionCSV) -> Self {
        let difficulty = u8::from(data.max_difficulty);
        CrownData {
            max_difficulty: data.max_difficulty,
            crown_2: if difficulty >= 2 {
                NonZeroU32::new(data.crown_2)
            } else {
                None
            },
            crown_3: if difficulty >= 3 {
                NonZeroU32::new(data.crown_3)
            } else {
                None
            },
            crown_4: if difficulty >= 4 {
                NonZeroU32::new(data.crown_4)
            } else {
                None
            },
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
/// Stages that restriction applies to.
pub enum RestrictionStages {
    /// Applies to all stages.
    All,
    /// Applies to only stage with this id.
    One(u32),
}
impl From<i32> for RestrictionStages {
    fn from(value: i32) -> Self {
        match value {
            -1 => Self::All,
            #[allow(clippy::cast_sign_loss)]
            x if x >= 0 => Self::One(value as u32),
            // Can do this as checking >= 0 first
            _ => panic!("Negative restriction stages number that isn't -1."),
        }
    }
}

#[derive(Debug, PartialEq)]
/// Stage's restriction. Multiple fields can be active at once.
pub struct Restriction {
    /// Which stages the restriction applies to.
    pub stages_applied: RestrictionStages,
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
    pub charagroup: Option<CharaGroup>,
}
impl Restriction {
    /// Get restriction data from stage option csv.
    pub fn from_option_csv(value: &StageOptionCSV, version: &Version) -> Restriction {
        let charagroup: Option<CharaGroup> = NonZeroU32::new(value.charagroup).map(|value| {
            let charagroup_map = version.get_cached_file::<CharaGroups>();
            charagroup_map.get_charagroup(value.into()).unwrap().clone()
        });
        // I really can't be bothered to deal with the insane amount of lifetime
        // stuff that comes with not cloning, + it's rare you need to do this
        // and even when you do it's like 5 numbers at most.
        //
        // Might not actually be too difficult to do this with lifetimes but for
        // now this is enough.

        Self {
            stages_applied: value.stage_id.into(),
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

#[derive(Debug)]
/// Full Version-agnostic owned stage struct.
pub struct Stage {
    /// Unique identifier for stage.
    pub id: StageID,

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
    /// ID of animated base (Doge = 2).
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
    pub energy: Option<u32>,
    /// Base XP reward of stage.
    pub xp: Option<u32>,
    /// Rewards available.
    pub rewards: Option<StageRewards>,

    /// How the map resets once cleared.
    pub reset_type: ResetType,
    /// Max stage clears before map disappears.
    pub max_clears: Option<NonZeroU32>,
    /// Gauntlet cooldown.
    pub cooldown: Option<NonZeroU32>,
    /// Binary mask of the star difficulty.
    pub star_mask: Option<u16>,
    /// Crown difficulties of stage.
    pub crown_data: Option<CrownData>,

    /// EX map that invades the stage.
    pub ex_invasion: Option<u32>,
    /// Stage's restrictions.
    pub restrictions: Option<Vec<Restriction>>,
    /// Stage's rules.
    pub rules: Option<SpecialRule>,
    /// Stage's score bonuses.
    pub bonuses: Option<ScoreBonus>,
}
impl From<StageData<'_>> for Stage {
    fn from(data: StageData) -> Self {
        let map_stage_data = data.get_map_stage_data();
        let map_option_data = data.get_map_option_data();
        let ex_invasion = data.get_ex_option_data();
        let rules = data.get_special_rules_data().cloned();
        let bonuses = data.get_score_bonus_data().cloned();
        log::debug!("Rules: {rules:?}");
        log::debug!("Bonuses: {bonuses:?}");
        // debug to reveal how the stage's rule works, useful for updates which
        // add a new rule

        let restrictions: Option<Vec<Restriction>>;
        if let Some(option_data) = data.get_stage_option_data() {
            restrictions = Some(
                option_data
                    .into_iter()
                    .map(|r| Restriction::from_option_csv(r, data.version()))
                    .collect(),
            );
        } else {
            restrictions = None;
        }

        let id = data.id;

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

        let width: u32 = data.stage_csv_data.line2.width;
        let base_hp: u32 = data.stage_csv_data.line2.base_hp;
        let max_enemies: u32 = data.stage_csv_data.line2.max_enemies;
        let anim_base_id: Option<NonZeroU32> =
            NonZeroU32::new(data.stage_csv_data.line2.anim_base_id);
        let time_limit: Option<NonZeroU32> = NonZeroU32::new(data.stage_csv_data.line2.time_limit);
        let is_base_indestructible: bool = u8_to_bool(data.stage_csv_data.line2.indestructible);
        let background_id: u32 = data.stage_csv_data.line2.background_id;
        let enemies: Vec<StageEnemy> = data
            .stage_csv_data
            .enemies
            .into_iter()
            .map(|e| StageEnemy::new(&e, data.stage_csv_data.line2.anim_base_id))
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

        let reset_type: ResetType;
        let max_clears: Option<NonZeroU32>;
        let cooldown: Option<NonZeroU32>;
        let star_mask: Option<u16>;
        let crown_data: Option<CrownData>;

        if let Some(data) = map_option_data {
            reset_type = ResetType::from(data.reset_type);
            max_clears = NonZeroU32::new(data.max_clears);
            cooldown = NonZeroU32::new(data.cooldown);
            star_mask = Some(data.star_mask);
            crown_data = Some(CrownData::from(&data));
        } else {
            reset_type = ResetType::None;
            max_clears = None;
            cooldown = None;
            star_mask = None;
            crown_data = None;
        }

        Self {
            id,
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

            reset_type,
            max_clears,
            cooldown,
            star_mask,
            crown_data,

            ex_invasion,
            restrictions,
            rules,
            bonuses,
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
    pub fn from_selector(selector: &str, version: &Version) -> Result<Self, FromSelectorError> {
        Self::from_id(parse_general_stage_id(selector)?, version).map_err(From::from)
    }

    /// Create new stage object from id.
    pub fn from_id(id: StageID, version: &Version) -> Result<Self, FullCSVError> {
        Ok(StageData::from_id(id, version)?.into())
    }

    /// Create a new stage object from `selector` in current version.
    #[cfg(test)]
    pub fn from_id_current(id: StageID) -> Result<Self, FullCSVError> {
        use crate::TEST_CONFIG;
        Self::from_id(id, TEST_CONFIG.version.current_version())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        TEST_CONFIG,
        game_data::{
            meta::stage::{
                stage_types::transform::transform_stage::stage_data_file,
                variant::StageVariantID as T,
            },
            stage::stage_util::get_stage_files,
        },
    };

    // test none values, esp. with crown data

    #[test]
    #[ignore]
    fn get_all() {
        let version = TEST_CONFIG.version.current_version();
        for stage_name in get_stage_files(version) {
            let stage = Stage::from_selector(&stage_name, version).unwrap();
            assert_eq!(stage_data_file(&stage.id), stage_name);
        }
    }

    #[test]
    fn test_labyrinth() {
        let labyrinth_stage_1 =
            Stage::from_id_current(StageID::from_components(T::Labyrinth, 0, 0)).unwrap();
        assert_eq!(labyrinth_stage_1.energy, None);
        assert_eq!(labyrinth_stage_1.star_mask, Some(448));
        assert_eq!(labyrinth_stage_1.restrictions, None);
    }
}
