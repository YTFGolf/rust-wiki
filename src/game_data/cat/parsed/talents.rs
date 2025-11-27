//! Parsed talents object.

use crate::game_data::cat::raw::talents::{TalentGroup, TalentLine};
use std::num::NonZeroUsize;
use strum::FromRepr;

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
impl TalentTargets {
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
/// A single talent.
pub struct SingleTalent {
    /// ID of the ability the talent corresponds to.
    pub ability_id: NonZeroUsize,
    /// Max level the talent can be upgraded to (0 = 1 for some reason).
    pub max_level: u8,
    /// Parameters of talent.
    pub params: Vec<(u16, u16)>,
    /// SkillDescriptions.csv id of talent.
    pub skill_description_id: usize,
    /// SkillAcquisition cost id.
    pub skill_costs_id: usize,
    /// Something to do with unlocking new targets.
    pub name_id_or_something: i16,
    /// Is it a normal or an ultra talent.
    pub ttype: TalentType,
}
impl SingleTalent {
    /// Get single talent from raw talent group. Returns `None` if the talent's
    /// ID is 0.
    pub fn from_raw(group: &TalentGroup) -> Option<Self> {
        let ability_id = NonZeroUsize::new(group.ability_id_x.into())?;

        let max_level = group.maxlv_x;
        let mut params = vec![];
        for (min, max) in [
            (group.min_x1, group.max_x1),
            (group.min_x2, group.max_x2),
            (group.min_x3, group.max_x3),
            (group.min_x4, group.max_x4),
        ] {
            if min == 0 && max == 0 {
                continue;
            }
            params.push((min, max));
        }

        let skill_description_id = group.text_id_x.into();
        let skill_costs_id = group.lv_id_x.into();
        let name_id_or_something = group.name_id_x;
        let ttype = TalentType::from_repr(group.limit_x).unwrap();

        let t = SingleTalent {
            ability_id,
            max_level,
            params,
            skill_description_id,
            skill_costs_id,
            name_id_or_something,
            ttype,
        };

        Some(t)
    }
}

#[derive(Debug)]
/// Talents that a unit has access to.
pub struct Talents {
    /// Unit that has these talents.
    pub unit_id: usize,
    /// Targets unlocked by unlocking certain talents.
    pub implicit_targets: Vec<TalentTargets>,
    /// Normal talents.
    pub normal: Vec<SingleTalent>,
    /// Ultra talents.
    pub ultra: Vec<SingleTalent>,
}
impl Talents {
    /// Convert raw talent line to [`Talents`].
    pub fn from_raw(raw: &TalentLine) -> Self {
        let unit_id = raw.fixed.id.into();
        let implicit_targets = TalentTargets::get_targets(raw.fixed.type_id);

        let mut normal = vec![];
        let mut ultra = vec![];

        for group in raw.groups.iter() {
            let Some(t) = SingleTalent::from_raw(group) else {
                continue;
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
    use super::*;
    use crate::{
        TEST_CONFIG,
        game_data::cat::raw::{talents::TalentsContainer, talents_cost::TalentsCostContainer},
    };

    #[test]
    fn check_all_talents() {
        let version = TEST_CONFIG.version.current_version();
        let talents_cont = version.get_cached_file::<TalentsContainer>();
        let talents_cost_cont = version.get_cached_file::<TalentsCostContainer>();
        for talents in talents_cont.iter() {
            for group in &talents.groups {
                let Some(talent) = SingleTalent::from_raw(group) else {
                    continue;
                };

                let costs = talents_cost_cont
                    .from_cost_id(talent.skill_costs_id)
                    .unwrap();
                assert!(
                    talent.max_level as usize <= costs.costs.len(),
                    "Mismatch for costs on unit {}",
                    talents.fixed.id
                );
                // make sure that unit actually has a defined cost for every
                // level

                if talent.name_id_or_something != -1 {
                    assert_ne!(talents.fixed.type_id, 0)
                }
                // if name_id is not -1 then this adds a new target; make sure
                // that type_id is filled in if that is the case

                // println!("{:?}", talent);
            }
        }
    }

    #[test]
    fn type_ids() {
        let version = TEST_CONFIG.version.current_version();
        let talents_cont = version.get_cached_file::<TalentsContainer>();

        let mr = talents_cont.from_id(11).unwrap();
        assert_eq!(
            TalentTargets::get_targets(mr.fixed.type_id),
            vec![TalentTargets::Zombie]
        );

        let mola_king = talents_cont.from_id(174).unwrap();
        assert_eq!(
            TalentTargets::get_targets(mola_king.fixed.type_id),
            vec![
                TalentTargets::MaybeRed,
                TalentTargets::MaybeFloating,
                TalentTargets::MaybeBlack,
                TalentTargets::Metal,
                TalentTargets::MaybeAngel,
                TalentTargets::Alien,
                TalentTargets::Zombie,
                TalentTargets::Relic,
                TalentTargets::MaybeTraitless,
                TalentTargets::MaybeAku
            ]
        );
    }
}
