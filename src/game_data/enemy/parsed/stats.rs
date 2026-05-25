//! Get enemy's stats.

use crate::game_data::{
    cat::{ability::Ability, parsed::stats::form::Attack},
    enemy::raw::stats::EnemyCSV,
    unit::traits::EnemyType,
};

#[derive(Debug)]
/// Stats of an enemy.
pub struct EnemyStats {
    /// Unit HP.
    pub hp: u32,
    /// HP knockbacks.
    pub kb: u32,
    // /// Death soul animation, more testing needs to be done.
    // pub death_anim: Option<NonZero<i8>>,
    /// Speed (distance travelled every frame).
    pub speed: u16,
    /// Base money drop.
    pub money_drop: u16,
    /// Unit attack.
    pub attack: Attack,
    /// All unit's abilities.
    pub abilities: Vec<Ability>,
    /// Unit's traits
    pub traits: Vec<EnemyType>,
}

impl EnemyStats {
    /// Enemy stats from raw CSV and animation data.
    pub fn from_raw(stats: &EnemyCSV) -> EnemyStats {
        Self {
            hp: stats.hp,
            kb: stats.kb,
            // death_anim: NonZero::new(stats.death),
            speed: stats.speed,
            money_drop: stats.money_drop,
            attack: todo!(),    //Attack::from_combined(combined),
            abilities: todo!(), //Ability::get_all_abilities(combined),
            traits: EnemyType::get_all_traits(stats),
        }
    }
}
