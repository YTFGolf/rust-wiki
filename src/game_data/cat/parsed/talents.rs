//! Parsed talents object.

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
}

#[cfg(test)]
mod tests {
    use crate::{
        TEST_CONFIG, game_data::cat::raw::talents::TalentsContainer,
        wiki_data::talent_names::TALENT_DATA,
    };

    use super::*;

    #[test]
    fn check_all_talents() {
        let version = TEST_CONFIG.version.current_version();
        let talents_cont = version.get_cached_file::<TalentsContainer>();
        for talents in talents_cont.iter() {
            if talents.fixed.type_id == 0 {
                continue;
            }

            (0..12).for_each(|i| {
                let target_bit = talents.fixed.type_id & (1 << i);
                // will be 0 if bit i is 0, 2^i if bit i is 1
                if target_bit == 0 {
                    return;
                }

                let target = TalentTargets::from_repr(i);

                println!("{target:?}");
            });

            // println!("{}", 0b100111111111);
            // println!("{:#b}", 2559);
            // println!("{:?}",2559.);
            println!("{:?}", talents.fixed);
            for talent in talents.groups.iter() {
                if talent.abilityID_X == 0 {
                    continue;
                }

                // if talent.nameID_X == -1 {
                //     continue;
                // }

                println!("{talent:?}");
                println!(
                    "abilityID_X = {:?}",
                    TALENT_DATA.get_talent_name(talent.abilityID_X.into())
                );
                let t = match talent.limit_X {
                    0 => "Normal",
                    1 => "Ultra",
                    _ => unreachable!(),
                };
                println!("{t} talent");

                /*
                                impl Display for EnemyType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let t = match self {
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
                        };
                        write!(f, "{t}")
                    }
                } */
            }

            // println!("{talents:?}");
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
