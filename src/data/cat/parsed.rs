#![allow(missing_docs, unused_imports, dead_code, unreachable_code)]

use super::{ability::Ability, raw::CombinedCatData};
use std::rc::Rc;

#[derive(Debug)]
pub enum EnemyType {
    Red,
    Float,
    Black,
    Metal,
    Traitless,
    Angel,
    Alien,
    Zombie,
    Relic,
    Aku,
}

#[derive(Debug)]
pub enum AttackRangeClassification {
    Normal,
    LD,
    Omni,
}

#[derive(Debug)]
pub struct AttackRange {
    classification: AttackRangeClassification,
    base: u16,
    distance: i16,
}

#[derive(Debug)]
pub struct AttackHit {
    active_ability: bool,
    damage: u32,
    range: AttackRange,
    foreswing: u16,
}

#[derive(Debug)]
pub enum AttackHits {
    Single([AttackHit; 1]),
    Double([AttackHit; 2]),
    Triple([AttackHit; 3]),
}

#[derive(Debug)]
pub enum AreaOfEffect {
    SingleAttack,
    AreaAttack,
}

#[derive(Debug)]
pub struct Attack {
    abilities: Rc<[Ability]>,
    targets: Rc<[EnemyType]>,
    hits: AttackHits,
    aoe: AreaOfEffect,
    standing_range: u16,
    tba: u16,
    backswing: u32,
}

#[derive(Debug)]
struct Cat {
    base_hp: u32,
    kb: u16,
    death_anim: i8,
    speed: u8,
    price: u16,
    respawn: u16,
    attack: Attack,
}

impl Cat {
    fn from_combined(_data: &CombinedCatData) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::TEST_CONFIG, data::cat::raw::read_data_file};

    #[test]
    fn tmp() {
        #[allow(unused_variables)]
        let cond = true;
        // let cond = false;
        if cond {
            return;
        }
        let file_name = "unit544.csv";
        let version = TEST_CONFIG.version.current_version();
        panic!(
            "{:#?}",
            read_data_file(file_name, version)
                .map(|comb| Cat::from_combined(&comb))
                .collect::<Vec<_>>()
        )
    }
}
