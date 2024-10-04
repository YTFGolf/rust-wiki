//! Module that deals with getting information about enemies in stages.
use crate::data::stage::stage_data::csv_types::StageEnemyCSV;
use either::Either::{self, Left, Right};

#[derive(Debug, PartialEq)]
/// Type of boss.
pub enum BossType {
    /// Isn't a boss.
    None,
    /// Normal boss.
    Boss,
    /// Screen shake boss.
    ScreenShake,
}
impl From<u32> for BossType {
    fn from(n: u32) -> Self {
        match n {
            0 => BossType::None,
            1 => BossType::Boss,
            2 => BossType::ScreenShake,
            _ => panic!("Unrecognised boss type value: {n}!"),
        }
    }
}

#[derive(Debug, PartialEq)]
/// Amount of the enemy that spawns.
pub enum EnemyAmount {
    /// Infinite.
    Infinite,
    /// Limited.
    Limit(std::num::NonZeroU32),
}
impl From<u32> for EnemyAmount {
    fn from(value: u32) -> Self {
        match std::num::NonZeroU32::new(value) {
            None => Self::Infinite,
            Some(n) => Self::Limit(n),
        }
    }
}

#[derive(Debug)]
/// Representation of an enemy in a stage.
pub struct StageEnemy {
    /// Wiki id (Doge is 0).
    pub id: u32,
    /// Amount.
    pub amount: EnemyAmount,
    /// Start frame.
    pub start_frame: u32,
    /// Do you enforce `start_frame` even if enemy spawns after base hit.
    pub enforce_start_frame: bool,
    /// Respawn time in frames.
    pub respawn_time: (u32, u32),
    /// Note: can be above 100%. Also for Dojo this is absolute damage, not
    /// percentage.
    pub base_hp: u32,
    /// Probably can go unused.
    pub layer: (u32, u32),
    /// Type of boss.
    pub boss_type: BossType,
    /// Is this enemy an animated base.
    pub is_base: bool,
    /// Either magnification or (hp, ap).
    pub magnification: Either<u32, (u32, u32)>,
    /// How many cats die before enemy appears.
    pub kill_count: Option<std::num::NonZeroU32>,
}

impl StageEnemy {
    /// Create new StageEnemy out of raw data.
    pub fn new(old: StageEnemyCSV, anim_base_id: u32) -> Self {
        let id = old.num - 2;
        let amount = old.amt.into();

        let start_frame = old.start_frame * 2;
        let enforce_start_frame = match old.is_spawn_delay {
            None => false,
            Some(b) => match b {
                0 => false,
                1 => true,
                _ => panic!("Value {b} is not a valid is_spawn_delay flag!"),
            },
        };

        let respawn_time = (old.respawn_frame_min * 2, old.respawn_frame_max * 2);
        let base_hp = old.base_hp;
        let layer = (old.layer_min, old.layer_max);
        let boss_type = old.boss_type.into();
        let is_base = old.num == anim_base_id;

        let hpmag = old.magnification.unwrap_or(100);
        let magnification = match old.attack_magnification {
            None => Left(hpmag),
            Some(a) => match a {
                0 => Left(hpmag),
                _ => Right((hpmag, a)),
            },
        };

        let kill_count = std::num::NonZeroU32::new(old.kill_count.unwrap_or(0));

        Self {
            id,
            amount,
            start_frame,
            enforce_start_frame,
            respawn_time,
            base_hp,
            layer,
            boss_type,
            is_base,
            magnification,
            kill_count,
        }
    }
}
