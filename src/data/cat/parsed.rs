//! High-level container for cat data.

use super::{ability::Ability, raw::CombinedCatData};
use std::{num::NonZero, rc::Rc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
/// Enemy types that can be targeted.
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
impl EnemyType {
    /// Get all of the cat's targets.
    pub fn get_all_targets(combined: &CombinedCatData) -> Vec<EnemyType> {
        let (fixed, variable) = combined;
        let mut targets = vec![];

        if bool(fixed.targ_red).unwrap() {
            targets.push(Self::Red)
        }
        if bool(fixed.targ_float).unwrap() {
            targets.push(Self::Float)
        }
        if bool(fixed.targ_black).unwrap() {
            targets.push(Self::Black)
        }
        if bool(fixed.targ_metal).unwrap() {
            targets.push(Self::Metal)
        }
        if bool(fixed.targ_traitless).unwrap() {
            targets.push(Self::Traitless)
        }
        if bool(fixed.targ_angel).unwrap() {
            targets.push(Self::Angel)
        }
        if bool(fixed.targ_alien).unwrap() {
            targets.push(Self::Alien)
        }
        if bool(fixed.targ_zombie).unwrap() {
            targets.push(Self::Zombie)
        }
        if bool(variable.targ_relic).unwrap() {
            targets.push(Self::Relic)
        }
        if bool(variable.targ_aku).unwrap() {
            targets.push(Self::Aku)
        }

        targets
    }
}
fn bool(value: u8) -> Result<bool, String> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        x => Err(format!("{x} is not a valid boolean number!")),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Range of an attack.
pub enum AttackRange {
    /// Range is standing range.
    Normal,
    /// LD.
    LD {
        /// Distance to base.
        base: i16,
        /// Area of effect.
        distance: i16,
    },
    /// Omnistrike.
    Omni {
        /// Distance to base.
        base: i16,
        /// Area of effect.
        distance: i16,
    },
    /// Same as hit 1.
    Unchanged,
}
impl AttackRange {
    const fn new(base: i16, distance: i16) -> Self {
        if base == 0 {
            AttackRange::Normal
        } else if distance > 0 {
            AttackRange::LD { base, distance }
        } else {
            AttackRange::Omni { base, distance }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Single hit of the unit's attack.
pub struct AttackHit {
    /// Is the ability active on this hit.
    pub active_ability: bool,
    /// Base damage of this hit.
    pub damage: u32,
    /// Range of this hit.
    pub range: AttackRange,
    /// Foreswing of this hit.
    pub foreswing: u16,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// The unit's attacks.
pub enum AttackHits {
    /// One attack.
    Single([AttackHit; 1]),
    /// Two attack.
    Double([AttackHit; 2]),
    /// Three attack.
    Triple([AttackHit; 3]),
}
impl AttackHits {
    fn from_combined(combined: &CombinedCatData) -> AttackHits {
        let (_, var) = combined;
        if var.mhit_atk2 == 0 {
            Self::Single([Self::single(combined)])
        } else if var.mhit_atk3 == 0 {
            Self::Double([Self::get_hit1(combined), Self::get_hit2(combined)])
        } else {
            Self::Triple([
                Self::get_hit1(combined),
                Self::get_hit2(combined),
                Self::get_hit3(combined),
            ])
        }
    }

    /// Only one attack hit.
    fn single(combined: &CombinedCatData) -> AttackHit {
        let mut hit = Self::get_hit1(combined);
        hit.active_ability = true;
        // assumption that it doesn't really matter here, might do some logging
        hit
    }

    /// Get the first attack hit. This is almost exactly the same as
    /// [`Self::single`], but it also takes into account the `proc_on_hit1`
    /// flag.
    fn get_hit1(combined: &CombinedCatData) -> AttackHit {
        let (fixed, variable) = combined;
        let active_ability = bool(variable.proc_on_hit1).unwrap();
        let damage = fixed.atk;
        let range = AttackRange::new(fixed.ld_base, fixed.ld_range);
        let foreswing = fixed.foreswing;
        AttackHit {
            active_ability,
            damage,
            range,
            foreswing,
        }
    }

    fn get_hit2(combined: &CombinedCatData) -> AttackHit {
        let (_, variable) = combined;
        let active_ability = bool(variable.proc_on_hit2).unwrap();
        let damage = variable.mhit_atk2;

        let range = match bool(variable.second_ld_is_different).unwrap() {
            true => AttackRange::new(variable.second_ld_base, variable.second_ld_range),
            false => AttackRange::Unchanged,
        };
        let foreswing = variable.mhit_atk2_fswing;

        AttackHit {
            active_ability,
            damage,
            range,
            foreswing,
        }
    }

    fn get_hit3(combined: &CombinedCatData) -> AttackHit {
        let (_, variable) = combined;
        let active_ability = bool(variable.proc_on_hit3).unwrap();
        let damage = variable.mhit_atk3;

        let range = match bool(variable.third_ld_is_different).unwrap() {
            true => AttackRange::new(variable.third_ld_base, variable.third_ld_range),
            false => AttackRange::Unchanged,
        };
        let foreswing = variable.mhit_atk3_fswing;

        AttackHit {
            active_ability,
            damage,
            range,
            foreswing,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Area of the unit's hits.
pub enum AreaOfEffect {
    /// First enemy in range.
    SingleAttack,
    /// All enemies in range.
    AreaAttack,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Unit's attack.
pub struct Attack {
    /// All hits of the unit's attack.
    pub hits: AttackHits,
    /// Attack area of effect.
    pub aoe: AreaOfEffect,
    /// Standing range before attack.
    pub standing_range: u16,
    /// Time between attacks.
    ///
    /// This is an interval, so cycle is `foreswing + max(backswing, 2 * tba -
    /// 1)`. Backswing is not a stat, it is the length of the unit's animation.
    pub tba: u16,
}
impl Attack {
    fn from_combined(combined: &CombinedCatData) -> Self {
        let (fixed, _) = combined;
        // could possibly use strum here
        let aoe = match fixed.is_area {
            0 => AreaOfEffect::SingleAttack,
            1 => AreaOfEffect::AreaAttack,
            _ => unreachable!(),
        };
        Self {
            hits: AttackHits::from_combined(combined),
            aoe,
            standing_range: fixed.range,
            tba: fixed.tba,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Stats at level 1 with no treasures.
pub struct CatStats {
    /// Unit HP.
    pub hp: u32,
    /// HP knockbacks.
    pub kb: u16,
    /// Death soul animation, more testing needs to be done.
    pub death_anim: Option<NonZero<i8>>,
    /// Speed (distance travelled every frame).
    pub speed: u8,
    /// EoC1 cost.
    pub price: u16,
    /// Respawn frames / 2.
    pub respawn_half: u16,
    /// Unit attack.
    pub attack: Attack,
    /// All unit's abilities.
    pub abilities: Rc<[Ability]>,
    /// Enemy types the unit targets.
    pub targets: Rc<[EnemyType]>,
}

impl CatStats {
    /// Get unit stats from the combined stat data.
    pub fn from_combined(combined: &CombinedCatData) -> Self {
        let (fixed, var) = combined;
        Self {
            hp: fixed.hp,
            kb: fixed.kb,
            death_anim: NonZero::new(var.death),
            speed: fixed.speed,
            price: fixed.price,
            respawn_half: fixed.respawn,
            attack: Attack::from_combined(combined),
            abilities: Ability::get_all_abilities(combined).into(),
            targets: EnemyType::get_all_targets(combined).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::TEST_CONFIG,
        data::cat::{
            ability::{Surge, SurgeType},
            raw::read_data_file,
        },
    };
    use Ability as A;
    use EnemyType as E;
    use std::iter::zip;

    fn get_unit(wiki_id: usize) -> impl Iterator<Item = CatStats> {
        let abs_id = wiki_id + 1;
        let file_name = format!("unit{abs_id:03}.csv");
        let combined_iter = read_data_file(&file_name, TEST_CONFIG.version.current_version());
        combined_iter.map(|combined| CatStats::from_combined(&combined))
    }

    fn sorted<T: Ord>(mut v: Vec<T>) -> Vec<T> {
        v.sort();
        v
    }

    #[test]
    fn test_bahamut() {
        let bahamut = get_unit(25);

        let forms = [
            CatStats {
                hp: 1500,
                kb: 3,
                death_anim: None,
                speed: 6,
                price: 3000,
                respawn_half: 2400,
                attack: Attack {
                    hits: AttackHits::Single([AttackHit {
                        active_ability: true,
                        damage: 5000,
                        range: AttackRange::Normal,
                        foreswing: 121,
                    }]),
                    aoe: AreaOfEffect::AreaAttack,
                    standing_range: 450,
                    tba: 240,
                },
                abilities: [].into(),
                targets: [].into(),
            },
            CatStats {
                hp: 1500,
                kb: 3,
                death_anim: None,
                speed: 6,
                price: 3000,
                respawn_half: 2400,
                attack: Attack {
                    hits: AttackHits::Single([AttackHit {
                        active_ability: true,
                        damage: 5000,
                        range: AttackRange::Normal,
                        foreswing: 121,
                    }]),
                    aoe: AreaOfEffect::AreaAttack,
                    standing_range: 450,
                    tba: 240,
                },
                abilities: [].into(),
                targets: [].into(),
            },
            CatStats {
                hp: 1500,
                kb: 6,
                death_anim: None,
                speed: 60,
                price: 3000,
                respawn_half: 1600,
                attack: Attack {
                    hits: AttackHits::Triple([
                        AttackHit {
                            active_ability: true,
                            damage: 5000,
                            range: AttackRange::Normal,
                            foreswing: 5,
                        },
                        AttackHit {
                            active_ability: false,
                            damage: 200,
                            range: AttackRange::Unchanged,
                            foreswing: 10,
                        },
                        AttackHit {
                            active_ability: false,
                            damage: 300,
                            range: AttackRange::Unchanged,
                            foreswing: 20,
                        },
                    ]),
                    aoe: AreaOfEffect::AreaAttack,
                    standing_range: 200,
                    tba: 0,
                },
                abilities: [].into(),
                targets: [].into(),
            },
        ];

        for (form, ans) in zip(bahamut, forms) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_dark_phono() {
        let dark_phono = get_unit(705);

        let forms = [
            CatStats {
                hp: 2600,
                kb: 5,
                death_anim: None,
                speed: 30,
                price: 3400,
                respawn_half: 2200,
                attack: Attack {
                    hits: AttackHits::Triple([
                        AttackHit {
                            active_ability: true,
                            damage: 700,
                            range: AttackRange::LD {
                                base: 200,
                                distance: 350,
                            },
                            foreswing: 70,
                        },
                        AttackHit {
                            active_ability: true,
                            damage: 700,
                            range: AttackRange::LD {
                                base: 400,
                                distance: 350,
                            },
                            foreswing: 80,
                        },
                        AttackHit {
                            active_ability: true,
                            damage: 700,
                            range: AttackRange::LD {
                                base: 490,
                                distance: 410,
                            },
                            foreswing: 90,
                        },
                    ]),
                    aoe: AreaOfEffect::AreaAttack,
                    standing_range: 500,
                    tba: 0,
                },
                abilities: sorted(vec![
                    A::Slow {
                        chance: 100,
                        duration: 60,
                    },
                    A::Surge(Surge {
                        stype: SurgeType::MiniSurge,
                        chance: 100,
                        spawn_quad: 1600,
                        range_quad: 2800,
                        level: 1,
                    }),
                    A::ImmuneToWave,
                    A::ImmuneToSurge,
                ])
                .into(),
                targets: [
                    E::Red,
                    E::Float,
                    E::Black,
                    E::Metal,
                    E::Traitless,
                    E::Angel,
                    E::Alien,
                    E::Zombie,
                    E::Relic,
                    E::Aku,
                ]
                .into(),
            },
            CatStats {
                hp: 3400,
                kb: 5,
                death_anim: None,
                speed: 30,
                price: 3400,
                respawn_half: 2200,
                attack: Attack {
                    hits: AttackHits::Triple([
                        AttackHit {
                            active_ability: true,
                            damage: 1000,
                            range: AttackRange::LD {
                                base: 200,
                                distance: 350,
                            },
                            foreswing: 70,
                        },
                        AttackHit {
                            active_ability: true,
                            damage: 1000,
                            range: AttackRange::LD {
                                base: 400,
                                distance: 350,
                            },
                            foreswing: 80,
                        },
                        AttackHit {
                            active_ability: true,
                            damage: 1000,
                            range: AttackRange::LD {
                                base: 490,
                                distance: 410,
                            },
                            foreswing: 90,
                        },
                    ]),
                    aoe: AreaOfEffect::AreaAttack,
                    standing_range: 500,
                    tba: 0,
                },
                abilities: sorted(vec![
                    A::Slow {
                        chance: 100,
                        duration: 60,
                    },
                    A::Surge(Surge {
                        stype: SurgeType::MiniSurge,
                        chance: 100,
                        spawn_quad: 1600,
                        range_quad: 2800,
                        level: 1,
                    }),
                    A::ImmuneToWave,
                    A::ImmuneToCurse,
                    A::ImmuneToSurge,
                ])
                .into(),
                targets: [
                    E::Red,
                    E::Float,
                    E::Black,
                    E::Metal,
                    E::Traitless,
                    E::Angel,
                    E::Alien,
                    E::Zombie,
                    E::Relic,
                    E::Aku,
                ]
                .into(),
            },
        ];

        for (form, ans) in zip(dark_phono, forms) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_dark_iz() {
        let bahamut = get_unit(25);

        let forms = [];

        for (form, ans) in zip(bahamut, forms) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    fn test_death_anim() {
        let bahamut = get_unit(25);
        // moneko

        let forms = [];

        for (form, ans) in zip(bahamut, forms) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    fn test_eva_02() {
        let bahamut = get_unit(25);

        let forms = [];

        for (form, ans) in zip(bahamut, forms) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    fn test_kamikaze() {
        let bahamut = get_unit(25);

        let forms = [];

        for (form, ans) in zip(bahamut, forms) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    fn test_omni() {
        let bahamut = get_unit(25);

        let forms = [];

        for (form, ans) in zip(bahamut, forms) {
            assert_eq!(form, ans);
        }
        todo!("cosmo")
    }
}
