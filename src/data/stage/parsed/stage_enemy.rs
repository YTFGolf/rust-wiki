//! Module that deals with getting information about enemies in stages.
use crate::data::stage::stage_data::csv_types::StageEnemyCSV;
use either::Either::{self, Left, Right};

#[derive(Debug)]
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

#[derive(Debug)]
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
    /// Respawn time.
    pub respawn_time: (u32, u32),
    /// Note: can be above 100%. Also for Dojo this is absolute damage, not
    /// percentage.
    pub base_hp: u32,
    /// Probably can go unused.
    pub layer: (u32, u32),
    /// Type of boss.
    pub boss_type: BossType,
    /// Either magnification or (hp, ap).
    pub magnification: Either<u32, (u32, u32)>,
    /// How many cats die before enemy appears.
    pub kill_count: Option<std::num::NonZeroU32>,
}

impl From<StageEnemyCSV> for StageEnemy {
    fn from(value: StageEnemyCSV) -> Self {
        let id = value.num - 2;
        let amount = value.amt.into();

        let start_frame = value.start_frame * 2;
        let enforce_start_frame = match value.is_spawn_delay {
            None => false,
            Some(b) => match b {
                0 => false,
                1 => true,
                _ => panic!("Value {b} is not a valid is_spawn_delay flag!"),
            },
        };

        let respawn_time = (value.respawn_frame_min * 2, value.respawn_frame_max * 2);
        let base_hp = value.base_hp;
        let layer = (value.layer_min, value.layer_max);
        let boss_type = value.boss_type.into();

        let hpmag = value.magnification.unwrap_or(100);
        let magnification = match value.attack_magnification {
            None => Left(hpmag),
            Some(a) => match a {
                0 => Left(hpmag),
                _ => Right((hpmag, a)),
            },
        };

        let kill_count = std::num::NonZeroU32::new(value.kill_count.unwrap_or(0));

        Self {
            id,
            amount,
            start_frame,
            enforce_start_frame,
            respawn_time,
            base_hp,
            layer,
            boss_type,
            magnification,
            kill_count,
        }
    }
}
