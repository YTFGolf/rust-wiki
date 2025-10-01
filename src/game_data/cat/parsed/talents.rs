//! Parsed talents object.

use std::num::NonZeroUsize;

use strum::FromRepr;

use crate::game_data::cat::raw::{
    talents::TalentLine,
    talents_cost::{TalentAcquisitionCost, TalentsCostContainer},
};

/// New targets that talents implicitly enable.
///
/// Only targets that appear in isolation can be determined. "Maybe" targets are
/// only used for Mola King. Unknown targets are not used at all (they're
/// probably witch and eva angel).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromRepr)]
#[allow(missing_docs)]
#[repr(usize)]
pub enum TalentTargets {
    MaybeRed = 0,
    MaybeFloating = 1,
    MaybeBlack = 2,
    Metal = 3,
    MaybeAngel = 4,
    Alien = 5,
    Zombie = 6,
    Relic = 7,
    MaybeTraitless = 8,
    Unknown1 = 9,
    Unknown2 = 10,
    MaybeAku = 11,

    /// Only use this to avoid having to panic in this module.
    AsYetUnknown = 99,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromRepr)]
#[repr(u8)]
/// Talent
pub enum TalentType {
    /// Level 30.
    Normal = 0,
    /// Level 60.
    Ultra = 1,
}

#[derive(Debug)]
struct SingleTalent {
    ability_id: NonZeroUsize,
    max_level: u8,
    params: Vec<(u16, u16)>,
    skill_description_id: usize,
    skill_costs: TalentAcquisitionCost,
    name_id_or_something: i16,
    ttype: TalentType,
}

#[derive(Debug)]
struct Talents {
    unit_id: usize,
    implicit_targets: Vec<TalentTargets>,
    normal: Vec<SingleTalent>,
    ultra: Vec<SingleTalent>,
}
impl Talents {
    fn get_targets(target_mask: u16) -> Vec<TalentTargets> {
        (0..12)
            .filter_map(|i| {
                let target_bit = target_mask & (1 << i);
                // will be 0 if bit i is 0, 2^i if bit i is 1
                if target_bit == 0 {
                    return None;
                }

                return Some(TalentTargets::from_repr(i).unwrap_or(TalentTargets::AsYetUnknown));
            })
            .collect()
    }

    pub fn from_raw(raw: &TalentLine, talents_cost_cont: &TalentsCostContainer) -> Self {
        let unit_id = raw.fixed.id.into();
        let implicit_targets = Self::get_targets(raw.fixed.type_id);

        let mut normal = vec![];
        let mut ultra = vec![];

        for group in raw.groups.iter() {
            let Some(ability_id) = NonZeroUsize::new(group.abilityID_X.into()) else {
                continue;
            };

            let max_level = group.MAXLv_X;
            let mut params = vec![];
            for (min, max) in [
                (group.min_X1, group.max_X1),
                (group.min_X2, group.max_X2),
                (group.min_X3, group.max_X3),
                (group.min_X4, group.max_X4),
            ] {
                if min == 0 && max == 0 {
                    continue;
                }
                params.push((min, max));
            }

            let skill_description_id = group.textID_X.into();
            let skill_costs = talents_cost_cont
                .from_cost_id(group.LvID_X.into())
                .unwrap()
                .clone();
            let name_id_or_something = group.nameID_X;
            let ttype = TalentType::from_repr(group.limit_X).unwrap();

            let t = SingleTalent {
                ability_id,
                max_level,
                params,
                skill_description_id,
                skill_costs,
                name_id_or_something,
                ttype,
            };

            match t.ttype {
                TalentType::Normal => normal.push(t),
                TalentType::Ultra => ultra.push(t),
            }
        }

        Self {
            unit_id,
            implicit_targets,
            normal,
            ultra,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        TEST_CONFIG,
        game_data::cat::raw::{talents::TalentsContainer, talents_cost::TalentsCostContainer},
        wiki_data::talent_names::TALENT_DATA,
    };

    use super::*;

    #[test]
    fn check_all_talents() {
        let version = TEST_CONFIG.version.current_version();
        let talents_cont = version.get_cached_file::<TalentsContainer>();
        let talents_cost_cont = version.get_cached_file::<TalentsCostContainer>();
        for talents in talents_cont.iter() {
            println!("{:?}", Talents::from_raw(talents, talents_cost_cont));
            println!("");
        }
        todo!()
    }
}

/*
tests:
Mr. - 11: upgrades type id to zombie
Mola King - 174: upgrades type id to all enemies

if type_id is not 0 then name_id must not be -1
if name_id is not -1 then type_id must not be 0
 */
