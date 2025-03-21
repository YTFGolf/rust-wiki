#![allow(missing_docs, unused_imports, dead_code)]

use super::raw::CombinedCatData;
use std::rc::Rc;

#[derive(Debug)]
pub enum Ability {}
// will need to be done better. Trait or proc macro I think
/*
/// Is it removed with curse.
is_cursable
/// Does this occur on every hit.
is_general
*/

#[derive(Debug)]
pub enum EnemyType {}

#[derive(Debug)]
pub struct AttackRange {}

#[derive(Debug)]
pub struct AttackHit {
    active_ability: bool,
    range: AttackRange,
    foreswing: u32,
    backswing: u32,
}

#[derive(Debug)]
pub enum AttackHits {
    Single([AttackHit; 1]),
    Double([AttackHit; 2]),
    Triple([AttackHit; 3]),
}

#[derive(Debug)]
pub struct Attack {
    abilities: Rc<[Ability]>,
    targets: Rc<[EnemyType]>,
    hits: AttackHits,
}

#[derive(Debug)]
struct Cat {
    base_hp: u32,
    kb: u16,
    speed: u8,
    // atk: u32,
    // tba: u16,
    // range: u16,
    price: u16,
    // width: u16,
}

impl Cat {
    fn from_combined(_data: &CombinedCatData) -> Self {
        Self {
            base_hp: todo!(),
            kb: todo!(),
            speed: todo!(),
            price: todo!(),
        }
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
        let cond = false;
        if cond {
            return;
        }
        let file_name = "unit026.csv";
        let version = TEST_CONFIG.version.current_version();
        panic!(
            "{:#?}",
            read_data_file(file_name, version)
                .map(|comb| Cat::from_combined(&comb))
                .collect::<Vec<_>>()
        )
    }
}
