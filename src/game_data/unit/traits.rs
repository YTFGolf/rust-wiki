
use crate::game_data::unit::stats_util::bool;
use std::fmt::Display;
use crate::game_data::cat::raw::stats::CombinedCatData;

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
/// Enemy types that can be targeted.
pub enum EnemyType {
    /// Red.
    Red,
    /// Floating.
    Floating,
    /// Black.
    Black,
    /// Metal.
    Metal,
    /// Traitless.
    Traitless,
    /// Angel.
    Angel,
    /// Alien.
    Alien,
    /// Zombie.
    Zombie,
    /// Relic.
    Relic,
    /// Aku.
    Aku,
    // make sure that MAX_VALUE is up-to-date if adding anything new in
    // actually why not just use tests to enforce that
    // TODO
}
/// Latest entry (therefore highest numerically).
pub const LATEST_ENEMY_TYPE: EnemyType = EnemyType::Aku;

impl EnemyType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::Floating => "Floating",
            Self::Black => "Black",
            Self::Metal => "Metal",
            Self::Traitless => "Traitless",
            Self::Angel => "Angel",
            Self::Alien => "Alien",
            Self::Zombie => "Zombie",
            Self::Relic => "Relic",
            Self::Aku => "Aku",
        }
    }
}

impl Display for EnemyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = self.as_str();
        write!(f, "{t}")
    }
}
impl EnemyType {
    /// Get all of the cat's targets.
    pub fn get_all_targets(combined: &CombinedCatData) -> Vec<EnemyType> {
        let (fixed, variable) = combined;
        let mut targets = vec![];

        if bool(fixed.targ_red).unwrap() {
            targets.push(Self::Red);
        }
        if bool(fixed.targ_float).unwrap() {
            targets.push(Self::Floating);
        }
        if bool(fixed.targ_black).unwrap() {
            targets.push(Self::Black);
        }
        if bool(fixed.targ_metal).unwrap() {
            targets.push(Self::Metal);
        }
        if bool(fixed.targ_traitless).unwrap() {
            targets.push(Self::Traitless);
        }
        if bool(fixed.targ_angel).unwrap() {
            targets.push(Self::Angel);
        }
        if bool(fixed.targ_alien).unwrap() {
            targets.push(Self::Alien);
        }
        if bool(fixed.targ_zombie).unwrap() {
            targets.push(Self::Zombie);
        }
        if bool(variable.targ_relic).unwrap() {
            targets.push(Self::Relic);
        }
        if bool(variable.targ_aku).unwrap() {
            targets.push(Self::Aku);
        }

        targets
    }
}
