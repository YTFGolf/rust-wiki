//! Deal with enemy traits.

use crate::game_data::cat::raw::stats::CombinedCatData;
use crate::game_data::{enemy::raw::stats::EnemyCSV, unit::stats_util::bool};
use std::fmt::Display;

#[repr(usize)]
// TODO rename
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
/// Enemy types that can be targeted.
pub enum EnemyType {
    Red,
    Floating,
    Dark,
    Metal,
    Traitless,
    Angel,
    Alien,
    Zombie,
    Witch,
    Typeless,
    StarredAlien,
    EvaAngel,
    Relic,
    Aku,
    Colossus,
    Behemoth,
    Sage,
    Supervillain,
    // make sure that MAX_VALUE is up-to-date if adding anything new in
    // actually why not just use tests to enforce that
    // TODO
}
/// Latest entry (therefore highest numerically).
pub const LATEST_ENEMY_TYPE: EnemyType = EnemyType::Supervillain;

impl EnemyType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::Floating => "Floating",
            Self::Dark => "Dark",
            Self::Metal => "Metal",
            Self::Traitless => "Traitless",
            Self::Angel => "Angel",
            Self::Alien => "Alien",
            Self::Zombie => "Zombie",
            Self::Witch => "Witch",
            Self::Typeless => "Typeless",
            Self::StarredAlien => "Starred Alien",
            Self::EvaAngel => "Eva Angel",
            Self::Relic => "Relic",
            Self::Aku => "Aku",
            Self::Colossus => "Colossus",
            Self::Behemoth => "Behemoth",
            Self::Sage => "Sage",
            Self::Supervillain => "Supervillain",
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
        if bool(fixed.targ_dark).unwrap() {
            targets.push(Self::Dark);
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

impl EnemyType {
    /// Get all of the enemy's traits.
    pub fn get_all_traits(stats: &EnemyCSV) -> Vec<EnemyType> {
        let mut targets = vec![];

        if bool(stats.red).unwrap() {
            targets.push(Self::Red);
        }
        if bool(stats.floating).unwrap() {
            targets.push(Self::Floating);
        }
        if bool(stats.dark).unwrap() {
            targets.push(Self::Dark);
        }
        if bool(stats.metal).unwrap() {
            targets.push(Self::Metal);
        }
        if bool(stats.traitless).unwrap() {
            targets.push(Self::Traitless);
        }
        if bool(stats.angel).unwrap() {
            targets.push(Self::Angel);
        }
        if bool(stats.alien).unwrap() {
            targets.push(Self::Alien);
        }
        if bool(stats.zombie).unwrap() {
            targets.push(Self::Zombie);
        }
        if bool(stats.witch).unwrap() {
            targets.push(Self::Witch)
        }
        if bool(stats.typeless).unwrap() {
            targets.push(Self::Typeless)
        }
        if bool(stats.starred_alien).unwrap() {
            targets.push(Self::StarredAlien)
        }
        if bool(stats.eva_angel).unwrap() {
            targets.push(Self::EvaAngel)
        }
        if bool(stats.relic).unwrap() {
            targets.push(Self::Relic);
        }
        if bool(stats.aku).unwrap() {
            targets.push(Self::Aku);
        }
        if bool(stats.colossus).unwrap() {
            targets.push(Self::Colossus)
        }
        if bool(stats.behemoth).unwrap() {
            targets.push(Self::Behemoth)
        }
        if bool(stats.sage).unwrap() {
            targets.push(Self::Sage)
        }
        if bool(stats.supervillain).unwrap() {
            targets.push(Self::Supervillain)
        }

        targets
    }
}
