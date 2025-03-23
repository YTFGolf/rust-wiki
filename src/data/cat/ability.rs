//! Deals with cat/enemy abilities.

use super::raw::CombinedCatData;

/// Configuration values/modifiers for abilities.
struct Config {
    /// Does the ability apply on every hit or only on specified ones (e.g.
    /// CgtG's KB vs double bounty)?
    is_general: bool,
    /// Is the ability removed by curse?
    is_cursable: bool,
}

type Percent = u8;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Possible type of wave attack.
pub enum WaveType {
    /// Normal wave.
    Wave,
    /// Mini wave, 2x speed and 20% damage.
    MiniWave,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Wave ability.
pub struct Wave {
    /// Type of wave.
    pub wtype: WaveType,
    /// Chance for wave to proc.
    pub chance: Percent,
    /// Level of wave (amount of hits).
    pub level: u8,
}

impl Wave {
    fn from_combined((fixed, var): &CombinedCatData) -> Self {
        let wtype = match var.is_mini_wave {
            0 => WaveType::Wave,
            1 => WaveType::MiniWave,
            x => panic!("Mini wave flag should be 0 or 1, got {x}"),
        };

        Self {
            wtype,
            chance: fixed.wave_chance,
            level: fixed.wave_level,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Possible type of surge attack.
pub enum SurgeType {
    /// Normal surge.
    Surge,
    /// Mini surge, 20% damage.
    MiniSurge,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Surge ability.
pub struct Surge {
    /// Type of surge.
    pub stype: SurgeType,
    /// Chance for surge to proc.
    pub chance: Percent,
    /// Surge range min bound (* 4 for some reason).
    pub spawn_quad: u16,
    /// Surge range distance (once again * 4).
    pub range_quad: u16,
    /// Level of surge (20f/1 tick per level),
    pub level: u8,
}

impl Surge {
    fn from_combined((_, var): &CombinedCatData) -> Self {
        let stype = match var.is_mini_surge {
            0 => SurgeType::Surge,
            1 => SurgeType::MiniSurge,
            x => panic!("Mini surge flag should be 0 or 1, got {x}"),
        };

        Self {
            stype,
            chance: var.surge_chance,
            spawn_quad: var.surge_spawn_quad,
            range_quad: var.surge_range_quad,
            level: var.surge_level,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Cat or enemy ability.
pub enum Ability {
    /// Strong against.
    StrongAgainst,
    /// Knockback.
    Knockback {
        /// Knockback chance.
        chance: Percent,
    },
    /// Freeze the enemy.
    Freeze {
        /// Chance to freeze.
        chance: Percent,
        /// Duration of freeze (f).
        duration: u16,
    },
    /// Slow the enemy.
    Slow {
        /// Chance to slow.
        chance: Percent,
        /// Duration of slow (f).
        duration: u16,
    },
    /// Resist.
    Resist,
    /// Massive damage.
    MassiveDamage,
    /// Critical hit.
    Crit {
        /// Crit chance.
        chance: Percent,
    },
    /// Targets only.
    TargetsOnly,
    /// Double money collected when defeating the enemy.
    DoubleBounty,
    /// Base destroyer.
    BaseDestroyer,
    /// Wave.
    Wave(Wave),
    /// Weaken.
    Weaken {
        /// Chance to weaken.
        chance: Percent,
        /// Duration of weaken (f).
        duration: u16,
        /// % of attack that opponent is weakened to.
        multiplier: Percent,
    },
    /// Strengthen at certain point.
    Strengthen {
        /// HP where it activates.
        hp: Percent,
        /// Extra damage percentage.
        multiplier: u16,
    },
    /// Survive lethal strike.
    Survives {
        /// Chance to survive.
        chance: Percent,
    },
    /// Metal.
    Metal,
    /// Immune to wave.
    ImmuneToWave,
    /// Wave blocker.
    WaveBlocker,
    /// Immune to knockback.
    ImmuneToKB,
    /// Immune to freeze.
    ImmuneToFreeze,
    /// Immune to slow.
    ImmuneToSlow,
    /// Immune to weaken.
    ImmuneToWeaken,
    /// Zombie killer.
    ZombieKiller,
    /// Witch killer.
    WitchKiller,
    /// Immune to boss shockwave.
    ImmuneToBossShockwave,
    /// Kamikaze.
    Kamikaze,
    /// Break starred alien barriers.
    BarrierBreaker {
        /// Chance to break barriers.
        chance: Percent,
    },
    /// Immune to warp.
    ImmuneToWarp,
    /// Eva Angel killer.
    EvaAngelKiller,
    /// Immune to curse.
    ImmuneToCurse,
    /// Insane resist.
    InsaneResist,
    /// Insane damage.
    InsaneDamage,
    /// Savage blow.
    SavageBlow {
        /// Savage blow chance.
        chance: Percent,
        /// Additional damage as a percent of initial.
        damage: u16,
    },
    /// Dodge.
    Dodge {
        /// Chance to dodge.
        chance: Percent,
        /// Duration of dodge (f).
        duration: u16,
    },
    /// Surge.
    Surge(Surge),
    /// Immune to toxic.
    ImmuneToToxic,
    /// Immune to surge.
    ImmuneToSurge,
    /// Curse.
    Curse {
        /// Chance to curse.
        chance: Percent,
        /// Duration of curse.
        duration: u16,
    },
    /// Shield pierce.
    ShieldPierce {
        /// Shield pierce chance.
        chance: Percent,
    },
    /// Colossus slayer.
    ColossusSlayer,
    /// Soulstrike.
    Soulstrike,
    /// Behemoth slayer.
    BehemothSlayer {
        /// Chance to dodge behemoth attacks.
        dodge_chance: Percent,
        /// Duration of dodge.
        dodge_duration: u16,
    },
    /// Counter surge.
    CounterSurge,
    /// Conjure a spirit.
    ConjureUnit {
        /// ID of the conjured spirit.
        id: u16,
    },
    /// Sage slayer.
    SageSlayer,
    /// Metal killer.
    MetalKiller {
        /// % of hp taken with every hit.
        damage: Percent,
    },
    /// Explosion attack.
    Explosion {
        /// Chance to explode.
        chance: Percent,
        /// Range. Appears to be spawnpoint * 4.
        spawn_quad: u16,
    },
    /// Immune to explosion.
    ImmuneToExplosion,
}

impl Ability {
    const fn get_config(&self) -> Config {
        match self {
            Self::StrongAgainst => Config {
                is_general: true,
                is_cursable: true,
            },
            Self::Knockback { .. } => Config {
                is_general: false,
                is_cursable: true,
            },
            Self::Freeze { .. } => Config {
                is_general: false,
                is_cursable: true,
            },
            Self::Slow { .. } => Config {
                is_general: false,
                is_cursable: true,
            },
            Self::Resist => Config {
                is_general: true,
                is_cursable: true,
            },
            Self::MassiveDamage => Config {
                is_general: true,
                is_cursable: true,
            },
            Self::Crit { .. } => Config {
                is_general: false,
                is_cursable: false,
            },
            Self::TargetsOnly => Config {
                is_general: true,
                is_cursable: true,
            },
            Self::DoubleBounty => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::BaseDestroyer => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::Wave(_) => Config {
                is_general: false,
                is_cursable: false,
            },
            Self::Weaken { .. } => Config {
                is_general: false,
                is_cursable: true,
            },
            Self::Strengthen { .. } => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::Survives { .. } => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::Metal => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ImmuneToWave => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::WaveBlocker => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ImmuneToKB => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ImmuneToFreeze => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ImmuneToSlow => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ImmuneToWeaken => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ZombieKiller => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::WitchKiller => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ImmuneToBossShockwave => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::Kamikaze => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::BarrierBreaker { .. } => Config {
                is_general: false,
                is_cursable: false,
            },
            Self::ImmuneToWarp => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::EvaAngelKiller => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ImmuneToCurse => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::InsaneResist => Config {
                is_general: true,
                is_cursable: true,
            },
            Self::InsaneDamage => Config {
                is_general: true,
                is_cursable: true,
            },
            Self::SavageBlow { .. } => Config {
                is_general: false,
                is_cursable: false,
            },
            Self::Dodge { .. } => Config {
                is_general: true,
                is_cursable: true,
            },
            Self::Surge(_) => Config {
                is_general: false,
                is_cursable: false,
            },
            Self::ImmuneToToxic => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ImmuneToSurge => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::Curse { .. } => Config {
                is_general: false,
                is_cursable: true,
            },
            Self::ShieldPierce { .. } => Config {
                is_general: false,
                is_cursable: false,
            },
            Self::ColossusSlayer => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::Soulstrike => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::BehemothSlayer { .. } => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::CounterSurge => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::ConjureUnit { .. } => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::SageSlayer => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::MetalKiller { .. } => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::Explosion { .. } => Config {
                is_general: false,
                is_cursable: false,
            },
            Self::ImmuneToExplosion => Config {
                is_general: true,
                is_cursable: false,
            },
        }
    }
}

impl Ability {
    /// Does the ability apply on every hit regardless?
    pub const fn is_general(&self) -> bool {
        self.get_config().is_general
    }

    /// Is the ability removed by curse?
    pub const fn is_cursable(&self) -> bool {
        self.get_config().is_cursable
    }

    /// Does the ability have targets when used on a cat? This is equivalent to
    /// [`Ability::is_cursable`].
    pub const fn has_targets(&self) -> bool {
        self.is_cursable()
    }
}

fn bool(value: Percent) -> Result<bool, String> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        x => Err(format!("{x} is not a valid boolean number!")),
    }
}

fn chance(value: Percent) -> bool {
    value > 0
}

impl Ability {
    /// Get the cat form's abilities as a vec.
    pub fn get_all_abilities(combined: &CombinedCatData) -> Vec<Ability> {
        let (fixed, variable) = combined;
        let mut abilities = vec![];

        if bool(fixed.has_strong).unwrap() {
            abilities.push(Self::StrongAgainst);
        }

        if chance(fixed.kb_chance) {
            abilities.push(Self::Knockback {
                chance: fixed.kb_chance,
            });
        }

        if chance(fixed.freeze_chance) {
            abilities.push(Self::Freeze {
                chance: fixed.freeze_chance,
                duration: fixed.freeze_duration,
            });
        }

        if chance(fixed.slow_chance) {
            abilities.push(Self::Slow {
                chance: fixed.slow_chance,
                duration: fixed.slow_duration,
            });
        }

        if bool(fixed.has_resist).unwrap() {
            abilities.push(Self::Resist);
        }

        if bool(fixed.has_massive_damage).unwrap() {
            abilities.push(Self::MassiveDamage);
        }

        if chance(fixed.crit_chance) {
            abilities.push(Self::Crit {
                chance: fixed.crit_chance,
            });
        }

        if bool(fixed.has_targets_only).unwrap() {
            abilities.push(Self::TargetsOnly);
        }

        if bool(fixed.has_double_bounty).unwrap() {
            abilities.push(Self::DoubleBounty);
        }

        if bool(fixed.has_double_bounty).unwrap() {
            abilities.push(Self::BaseDestroyer);
        }

        if chance(fixed.wave_chance) {
            abilities.push(Self::Wave(Wave::from_combined(combined)));
        }

        if chance(fixed.weaken_chance) {
            abilities.push(Self::Weaken {
                chance: fixed.weaken_chance,
                duration: fixed.weaken_duration,
                multiplier: fixed.weaken_multiplier,
            });
        }

        if fixed.strengthen_hp > 0 {
            abilities.push(Self::Strengthen {
                hp: fixed.strengthen_hp,
                multiplier: fixed.strengthen_multiplier,
            });
        }

        if chance(fixed.survives_chance) {
            abilities.push(Self::Survives {
                chance: fixed.survives_chance,
            });
        }

        if bool(fixed.has_metal).unwrap() {
            abilities.push(Self::Metal);
        }

        if bool(fixed.immune_wave).unwrap() {
            abilities.push(Self::ImmuneToWave);
        }

        if bool(fixed.has_wave_blocker).unwrap() {
            abilities.push(Self::WaveBlocker);
        }

        if bool(fixed.immune_kb).unwrap() {
            abilities.push(Self::ImmuneToKB);
        }

        if bool(fixed.immune_freeze).unwrap() {
            abilities.push(Self::ImmuneToFreeze);
        }

        if bool(fixed.immune_slow).unwrap() {
            abilities.push(Self::ImmuneToSlow);
        }

        if bool(fixed.immune_weaken).unwrap() {
            abilities.push(Self::ImmuneToWeaken);
        }

        if bool(variable.has_zombie_killer.unwrap_or_default()).unwrap() {
            abilities.push(Self::ZombieKiller);
        }

        if bool(variable.has_witch_killer).unwrap() {
            abilities.push(Self::WitchKiller);
        }

        if bool(variable.immune_boss_shockwave).unwrap() {
            abilities.push(Self::ImmuneToBossShockwave);
        }

        if variable.kamikaze > 0 {
            abilities.push(Self::Kamikaze);
        }

        if chance(variable.barrier_break_chance) {
            abilities.push(Self::BarrierBreaker {
                chance: variable.barrier_break_chance,
            });
        }

        if bool(variable.immune_warp).unwrap() {
            abilities.push(Self::ImmuneToWarp);
        }

        if bool(variable.has_eva_angel_killer).unwrap() {
            abilities.push(Self::EvaAngelKiller);
        }

        if bool(variable.immune_curse).unwrap() {
            abilities.push(Self::ImmuneToCurse);
        }

        if bool(variable.has_insane_resist).unwrap() {
            abilities.push(Self::InsaneResist);
        }

        if bool(variable.has_insane_damage).unwrap() {
            abilities.push(Self::InsaneDamage);
        }

        if chance(variable.savage_blow_chance) {
            abilities.push(Self::SavageBlow {
                chance: variable.savage_blow_chance,
                damage: variable.savage_blow_percent,
            });
        }

        if chance(variable.dodge_chance) {
            abilities.push(Self::Dodge {
                chance: variable.dodge_chance,
                duration: variable.dodge_duration,
            });
        }

        if chance(variable.surge_chance) {
            abilities.push(Self::Surge(Surge::from_combined(combined)));
        }

        if bool(variable.immune_toxic).unwrap() {
            abilities.push(Self::ImmuneToToxic);
        }

        if bool(variable.immune_surge).unwrap() {
            abilities.push(Self::ImmuneToSurge);
        }

        if chance(variable.curse_chance) {
            abilities.push(Self::Curse {
                chance: variable.curse_chance,
                duration: variable.curse_duration,
            });
        }

        if chance(variable.shield_pierce_chance) {
            abilities.push(Self::ShieldPierce {
                chance: variable.shield_pierce_chance,
            });
        }

        if bool(variable.has_colossus_slayer).unwrap() {
            abilities.push(Self::ColossusSlayer);
        }

        if bool(variable.has_soulstrike).unwrap() {
            abilities.push(Self::Soulstrike);
        }

        if bool(variable.has_behemoth_slayer).unwrap() {
            abilities.push(Self::BehemothSlayer {
                dodge_chance: variable.bslayer_dodge_chance,
                dodge_duration: variable.bslayer_dodge_duration,
            });
        }

        if bool(variable.has_counter_surge).unwrap() {
            abilities.push(Self::CounterSurge);
        }

        if variable.conjure_unit_id > 0 {
            abilities.push(Self::ConjureUnit {
                id: variable.conjure_unit_id as u16,
            });
        }

        if bool(variable.has_sage_slayer).unwrap() {
            abilities.push(Self::SageSlayer);
        }

        if variable.metal_killer_percent > 0 {
            abilities.push(Self::MetalKiller {
                damage: variable.metal_killer_percent,
            });
        }

        if chance(variable.explosion_chance) {
            abilities.push(Self::Explosion {
                chance: variable.explosion_chance,
                spawn_quad: variable.explosion_spawn_quad,
            });
        }

        if bool(variable.immune_explosion).unwrap() {
            abilities.push(Self::ImmuneToExplosion);
        }

        abilities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::TEST_CONFIG, data::cat::raw::read_data_file};
    use Ability as A;
    use std::iter::zip;

    fn get_unit(wiki_id: usize) -> impl Iterator<Item = Vec<Ability>> {
        let abs_id = wiki_id + 1;
        let file_name = format!("unit{abs_id:03}.csv");
        let combined_iter = read_data_file(&file_name, TEST_CONFIG.version.current_version());
        combined_iter.map(|combined| Ability::get_all_abilities(&combined))
    }

    fn sorted<T: Ord>(mut v: Vec<T>) -> Vec<T> {
        v.sort();
        v
    }

    #[test]
    fn test_no_abilities() {
        let cat = get_unit(0);
        for form in cat {
            assert_eq!(form, vec![]);
        }
    }

    #[test]
    fn test_no_abilities_multihit() {
        let bahamut = get_unit(25);
        for form in bahamut {
            assert_eq!(form, vec![]);
        }
    }

    #[test]
    fn test_surge() {
        let dasli = get_unit(543);

        let form_abilities = [
            vec![
                A::Surge(Surge {
                    stype: SurgeType::Surge,
                    chance: 100,
                    spawn_quad: 1600,
                    range_quad: 1200,
                    level: 2,
                }),
                A::Curse {
                    chance: 100,
                    duration: 135,
                },
                A::ImmuneToWave,
                A::ImmuneToWeaken,
                A::ImmuneToWarp,
                A::ImmuneToCurse,
                A::ImmuneToSurge,
            ],
            vec![
                A::Surge(Surge {
                    stype: SurgeType::Surge,
                    chance: 100,
                    spawn_quad: 1600,
                    range_quad: 1200,
                    level: 3,
                }),
                A::Curse {
                    chance: 100,
                    duration: 135,
                },
                A::ImmuneToWave,
                A::ImmuneToWeaken,
                A::ImmuneToWarp,
                A::ImmuneToCurse,
                A::ImmuneToSurge,
            ],
        ];

        for (form, ans) in zip(dasli, form_abilities) {
            assert_eq!(form, sorted(ans));
        }
    }

    #[test]
    fn test_m_killer() {
        let ultra_kaguya = get_unit(138).nth(3).unwrap();
        assert_eq!(
            ultra_kaguya,
            vec![
                A::Slow {
                    chance: 20,
                    duration: 70
                },
                A::MetalKiller { damage: 12 }
            ]
        );
    }

    #[test]

    fn test_explosion() {
        let dr_nova = get_unit(771);

        let ans = vec![
            A::Slow {
                chance: 100,
                duration: 100,
            },
            A::Curse {
                chance: 30,
                duration: 120,
            },
            A::SageSlayer,
            A::Explosion {
                chance: 100,
                spawn_quad: 1400,
            },
            A::ImmuneToCurse,
        ];
        let ans = sorted(ans);

        for form in dr_nova {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_basic_weaken() {
        let thaumaturge = get_unit(198).nth(2).unwrap();
        assert_eq!(
            thaumaturge,
            vec![
                A::Weaken {
                    chance: 100,
                    duration: 200,
                    multiplier: 50
                },
                A::ZombieKiller
            ]
        );
    }

    #[test]
    fn test_max_weaken() {
        let cat_jobs = get_unit(237);

        const WEAKEN: Ability = A::Weaken {
            chance: 100,
            duration: 150,
            multiplier: 1,
        };

        let form_abilities = [
            vec![WEAKEN],
            vec![WEAKEN],
            vec![
                WEAKEN,
                A::Survives { chance: 100 },
                A::Surge(Surge {
                    stype: SurgeType::Surge,
                    chance: 100,
                    spawn_quad: 600,
                    range_quad: 3600,
                    level: 1,
                }),
            ],
        ];

        for (form, ans) in zip(cat_jobs, form_abilities) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_lil_valk() {
        let lil_valk = get_unit(435);

        let ans = vec![
            A::ZombieKiller,
            A::BarrierBreaker { chance: 30 },
            A::ImmuneToWave,
            A::ImmuneToKB,
            A::ImmuneToFreeze,
            A::ImmuneToSlow,
            A::ImmuneToWeaken,
            A::ImmuneToWarp,
            A::ImmuneToCurse,
        ];
        let ans = sorted(ans);

        for form in lil_valk {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_gothic_mitama() {
        let goth_mit = get_unit(378);

        let form_abilities = [
            vec![
                A::Slow {
                    chance: 100,
                    duration: 100,
                },
                A::Resist,
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
            ],
            vec![
                A::Slow {
                    chance: 100,
                    duration: 100,
                },
                A::Resist,
                A::Weaken {
                    chance: 100,
                    duration: 100,
                    multiplier: 50,
                },
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
            ],
        ];

        for (form, ans) in zip(goth_mit, form_abilities) {
            assert_eq!(form, sorted(ans));
        }
    }

    #[test]
    fn test_lasvoss() {
        let lasvoss = get_unit(519);

        let form_abilities = [
            vec![
                A::Strengthen {
                    hp: 1,
                    multiplier: 50,
                },
                A::Survives { chance: 100 },
                A::SavageBlow {
                    chance: 30,
                    damage: 200,
                },
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
                A::ImmuneToWarp,
                A::ImmuneToCurse,
            ],
            vec![
                A::Strengthen {
                    hp: 1,
                    multiplier: 50,
                },
                A::Survives { chance: 100 },
                A::SavageBlow {
                    chance: 30,
                    damage: 200,
                },
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
                A::ImmuneToWarp,
                A::ImmuneToCurse,
            ],
            vec![
                A::Strengthen {
                    hp: 30,
                    multiplier: 50,
                },
                A::Survives { chance: 100 },
                A::SavageBlow {
                    chance: 30,
                    damage: 200,
                },
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
                A::ImmuneToWarp,
                A::ImmuneToCurse,
                A::ImmuneToToxic,
                A::ImmuneToSurge,
            ],
        ];

        for (form, ans) in zip(lasvoss, form_abilities) {
            assert_eq!(form, sorted(ans));
        }
    }

    #[test]
    fn test_doron() {
        let doron = get_unit(613);

        let form_abilities = [
            vec![
                A::Kamikaze,
                A::Freeze {
                    chance: 100,
                    duration: 150,
                },
                A::Surge(Surge {
                    stype: SurgeType::Surge,
                    chance: 100,
                    spawn_quad: 1600,
                    range_quad: 2400,
                    level: 1,
                }),
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
                A::ImmuneToWarp,
                A::ImmuneToCurse,
                A::ImmuneToSurge,
            ],
            vec![
                A::Kamikaze,
                A::Freeze {
                    chance: 100,
                    duration: 200,
                },
                A::Surge(Surge {
                    stype: SurgeType::Surge,
                    chance: 100,
                    spawn_quad: 1600,
                    range_quad: 2400,
                    level: 3,
                }),
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
                A::ImmuneToWarp,
                A::ImmuneToCurse,
                A::ImmuneToSurge,
            ],
            vec![
                A::Kamikaze,
                A::Knockback { chance: 100 },
                A::Freeze {
                    chance: 100,
                    duration: 200,
                },
                A::Surge(Surge {
                    stype: SurgeType::Surge,
                    chance: 100,
                    spawn_quad: 1600,
                    range_quad: 2400,
                    level: 3,
                }),
                A::ColossusSlayer,
                A::BehemothSlayer {
                    dodge_chance: 5,
                    dodge_duration: 30,
                },
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
                A::ImmuneToWarp,
                A::ImmuneToCurse,
                A::ImmuneToSurge,
            ],
        ];

        for (form, ans) in zip(doron, form_abilities) {
            assert_eq!(form, sorted(ans));
        }
    }

    #[test]
    fn test_iz_of_grief() {
        let iz_of_grief = get_unit(657);

        let form_abilities = [
            vec![
                A::MassiveDamage,
                A::Strengthen {
                    hp: 50,
                    multiplier: 100,
                },
                A::ZombieKiller,
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
                A::ImmuneToSurge,
            ],
            vec![
                A::MassiveDamage,
                A::Strengthen {
                    hp: 50,
                    multiplier: 150,
                },
                A::ZombieKiller,
                A::BarrierBreaker { chance: 100 },
                A::ShieldPierce { chance: 100 },
                A::ColossusSlayer,
                A::ImmuneToWave,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
                A::ImmuneToWarp,
                A::ImmuneToCurse,
                A::ImmuneToToxic,
                A::ImmuneToSurge,
            ],
        ];

        for (form, ans) in zip(iz_of_grief, form_abilities) {
            assert_eq!(form, sorted(ans));
        }
    }

    #[test]
    fn test_freeze() {
        let bombercat = get_unit(127);

        let form_abilities = [
            vec![A::Freeze {
                chance: 20,
                duration: 60,
            }],
            vec![A::Freeze {
                chance: 20,
                duration: 60,
            }],
            vec![A::Freeze {
                chance: 100,
                duration: 60,
            }],
        ];

        for (form, ans) in zip(bombercat, form_abilities) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_Courier() {
        let Courier = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(Courier, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_Cosmo_ultra() {
        let Cosmo_ultra = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(Cosmo_ultra, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_Matador() {
        let Matador = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(Matador, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_soulstrike() {
        let soulstrike = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(soulstrike, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_Colossus() {
        let Colossus = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(Colossus, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_Jianghsi() {
        let Jianghsi = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(Jianghsi, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_sblow() {
        let sblow = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(sblow, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_Bora_all_forms() {
        let Bora_all_forms = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(Bora_all_forms, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_Something_with_wave_and_mini_wave() {
        let Something_with_wave_and_mini_wave = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(Something_with_wave_and_mini_wave, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_Thief_or_rich_cat() {
        let Thief_or_cat_jobs = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(Thief_or_cat_jobs, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_aaaaaaaaaaa_CGtG() {
        let CGtG = get_unit(25);
        let form_abilities: [Vec<Ability>; 0] = [];

        for (form, ans) in zip(CGtG, form_abilities) {
            assert_eq!(form, ans);
        }
        todo!()
    }
}
