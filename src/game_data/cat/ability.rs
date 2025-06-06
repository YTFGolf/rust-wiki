//! Deals with cat/enemy abilities.

use super::raw::stats::CombinedCatData;
use strum::EnumIter;

type Percent = u8;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
/// Possible type of wave attack.
pub enum WaveType {
    /// Normal wave.
    #[default]
    Wave,
    /// Mini wave, 2x speed and 20% damage.
    MiniWave,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
/// Possible type of surge attack.
pub enum SurgeType {
    /// Normal surge.
    #[default]
    Surge,
    /// Mini surge, 20% damage.
    MiniSurge,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, EnumIter)]
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
    /// Does the ability apply at all times or must it be activated?
    pub const fn is_passive(&self) -> bool {
        match self {
            Self::StrongAgainst
            | Self::Resist
            | Self::MassiveDamage
            | Self::TargetsOnly
            | Self::DoubleBounty
            | Self::BaseDestroyer
            | Self::Strengthen { .. }
            | Self::Survives { .. }
            | Self::Metal
            | Self::ImmuneToWave
            | Self::WaveBlocker
            | Self::ImmuneToKB
            | Self::ImmuneToFreeze
            | Self::ImmuneToSlow
            | Self::ImmuneToWeaken
            | Self::ZombieKiller
            | Self::WitchKiller
            | Self::ImmuneToBossShockwave
            | Self::Kamikaze
            | Self::ImmuneToWarp
            | Self::EvaAngelKiller
            | Self::ImmuneToCurse
            | Self::InsaneResist
            | Self::InsaneDamage
            | Self::Dodge { .. }
            | Self::ImmuneToToxic
            | Self::ImmuneToSurge
            | Self::ColossusSlayer
            | Self::Soulstrike
            | Self::BehemothSlayer { .. }
            | Self::CounterSurge
            | Self::ConjureUnit { .. }
            | Self::SageSlayer
            | Self::MetalKiller { .. }
            | Self::ImmuneToExplosion => true,
            //
            Self::Knockback { .. }
            | Self::Freeze { .. }
            | Self::Slow { .. }
            | Self::Crit { .. }
            | Self::Wave(_)
            | Self::Weaken { .. }
            | Self::BarrierBreaker { .. }
            | Self::SavageBlow { .. }
            | Self::Surge(_)
            | Self::Curse { .. }
            | Self::ShieldPierce { .. }
            | Self::Explosion { .. } => false,
        }
    }

    /// Is the ability removed by curse?
    pub const fn is_cursable(&self) -> bool {
        match self {
            Self::StrongAgainst
            | Self::Knockback { .. }
            | Self::Freeze { .. }
            | Self::Slow { .. }
            | Self::Resist
            | Self::MassiveDamage
            | Self::TargetsOnly
            | Self::Weaken { .. }
            | Self::InsaneResist
            | Self::InsaneDamage
            | Self::Dodge { .. }
            | Self::Curse { .. } => true,
            //
            Self::Crit { .. }
            | Self::DoubleBounty
            | Self::BaseDestroyer
            | Self::Wave(_)
            | Self::Strengthen { .. }
            | Self::Survives { .. }
            | Self::Metal
            | Self::ImmuneToWave
            | Self::WaveBlocker
            | Self::ImmuneToKB
            | Self::ImmuneToFreeze
            | Self::ImmuneToSlow
            | Self::ImmuneToWeaken
            | Self::ZombieKiller
            | Self::WitchKiller
            | Self::ImmuneToBossShockwave
            | Self::Kamikaze
            | Self::BarrierBreaker { .. }
            | Self::ImmuneToWarp
            | Self::EvaAngelKiller
            | Self::ImmuneToCurse
            | Self::SavageBlow { .. }
            | Self::Surge(_)
            | Self::ImmuneToToxic
            | Self::ImmuneToSurge
            | Self::ShieldPierce { .. }
            | Self::ColossusSlayer
            | Self::Soulstrike
            | Self::BehemothSlayer { .. }
            | Self::CounterSurge
            | Self::ConjureUnit { .. }
            | Self::SageSlayer
            | Self::MetalKiller { .. }
            | Self::Explosion { .. }
            | Self::ImmuneToExplosion => false,
        }
    }

    /// Does the ability have targets when used on a cat? This is equivalent to
    /// [`Ability::is_cursable`].
    pub const fn has_targets(&self) -> bool {
        self.is_cursable()
    }

    /// Is the ability an immunity?
    pub const fn is_immunity(&self) -> bool {
        match self {
            Ability::StrongAgainst
            | Ability::Knockback { .. }
            | Ability::Freeze { .. }
            | Ability::Slow { .. }
            | Ability::Resist
            | Ability::MassiveDamage
            | Ability::Crit { .. }
            | Ability::TargetsOnly
            | Ability::DoubleBounty
            | Ability::BaseDestroyer
            | Ability::Wave(_)
            | Ability::Weaken { .. }
            | Ability::Strengthen { .. }
            | Ability::Survives { .. }
            | Ability::Metal
            | Ability::WaveBlocker
            | Ability::ZombieKiller
            | Ability::WitchKiller
            | Ability::Kamikaze
            | Ability::BarrierBreaker { .. }
            | Ability::EvaAngelKiller
            | Ability::InsaneResist
            | Ability::InsaneDamage
            | Ability::SavageBlow { .. }
            | Ability::Dodge { .. }
            | Ability::Surge(_)
            | Ability::Curse { .. }
            | Ability::ShieldPierce { .. }
            | Ability::ColossusSlayer
            | Ability::Soulstrike
            | Ability::BehemothSlayer { .. }
            | Ability::CounterSurge
            | Ability::ConjureUnit { .. }
            | Ability::SageSlayer
            | Ability::MetalKiller { .. }
            | Ability::Explosion { .. } => false,
            //
            Ability::ImmuneToWave
            | Ability::ImmuneToKB
            | Ability::ImmuneToFreeze
            | Ability::ImmuneToSlow
            | Ability::ImmuneToWeaken
            | Ability::ImmuneToBossShockwave
            | Ability::ImmuneToWarp
            | Ability::ImmuneToCurse
            | Ability::ImmuneToToxic
            | Ability::ImmuneToSurge
            | Ability::ImmuneToExplosion => true,
        }
    }
}
// Ability::StrongAgainst=>(),Ability::Knockback{..}=>(),Ability::Freeze{..}=>(),Ability::Slow{..}=>(),Ability::Resist=>(),Ability::MassiveDamage=>(),Ability::Crit{..}=>(),Ability::TargetsOnly=>(),Ability::DoubleBounty=>(),Ability::BaseDestroyer=>(),Ability::Wave(_)=>(),Ability::Weaken{..}=>(),Ability::Strengthen{..}=>(),Ability::Survives{..}=>(),Ability::Metal=>(),Ability::ImmuneToWave=>(),Ability::WaveBlocker=>(),Ability::ImmuneToKB=>(),Ability::ImmuneToFreeze=>(),Ability::ImmuneToSlow=>(),Ability::ImmuneToWeaken=>(),Ability::ZombieKiller=>(),Ability::WitchKiller=>(),Ability::ImmuneToBossShockwave=>(),Ability::Kamikaze=>(),Ability::BarrierBreaker{..}=>(),Ability::ImmuneToWarp=>(),Ability::EvaAngelKiller=>(),Ability::ImmuneToCurse=>(),Ability::InsaneResist=>(),Ability::InsaneDamage=>(),Ability::SavageBlow{..}=>(),Ability::Dodge{..}=>(),Ability::Surge(_)=>(),Ability::ImmuneToToxic=>(),Ability::ImmuneToSurge=>(),Ability::Curse{..}=>(),Ability::ShieldPierce{..}=>(),Ability::ColossusSlayer=>(),Ability::Soulstrike=>(),Ability::BehemothSlayer{..}=>(),Ability::CounterSurge=>(),Ability::ConjureUnit{..}=>(),Ability::SageSlayer=>(),Ability::MetalKiller{..}=>(),Ability::Explosion{..}=>(),Ability::ImmuneToExplosion=>(),

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

        if bool(fixed.has_base_destroyer).unwrap() {
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

        if variable.kamikaze != 0 {
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
            #[allow(clippy::cast_sign_loss)]
            // can cast as id > 0 when i16
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
    use crate::{TEST_CONFIG, game_data::cat::raw::stats::read_data_file};
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
    fn test_courier() {
        let courier = get_unit(658);

        let form_abilities = [
            vec![],
            vec![A::MassiveDamage],
            vec![
                A::MassiveDamage,
                A::BehemothSlayer {
                    dodge_chance: 5,
                    dodge_duration: 30,
                },
            ],
        ];

        for (form, ans) in zip(courier, form_abilities) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_cosmo() {
        let cosmo = get_unit(135);

        let form_abilities = [
            vec![A::Knockback { chance: 20 }],
            vec![A::Knockback { chance: 100 }],
            vec![
                A::Knockback { chance: 100 },
                A::Freeze {
                    chance: 100,
                    duration: 150,
                },
            ],
            vec![
                A::Knockback { chance: 100 },
                A::Freeze {
                    chance: 100,
                    duration: 150,
                },
                A::Wave(Wave {
                    wtype: WaveType::Wave,
                    chance: 100,
                    level: 10,
                }),
                A::SageSlayer,
            ],
        ];

        for (form, ans) in zip(cosmo, form_abilities) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_dodge() {
        let matador = get_unit(495);

        let ans = vec![A::Dodge {
            chance: 30,
            duration: 60,
        }];

        for form in matador {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_soulstrike() {
        let mighty_sphinx = get_unit(715);

        let ans = vec![
            A::Resist,
            A::Weaken {
                chance: 50,
                duration: 120,
                multiplier: 50,
            },
            A::ZombieKiller,
            A::Soulstrike,
        ];

        for form in mighty_sphinx {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_colossus_slayer() {
        let sirius = get_unit(686);

        let ans = vec![
            A::StrongAgainst,
            A::ColossusSlayer,
            A::ImmuneToKB,
            A::ImmuneToFreeze,
            A::ImmuneToSlow,
            A::ImmuneToWeaken,
            A::ImmuneToWarp,
            A::ImmuneToCurse,
            A::ImmuneToToxic,
        ];
        let ans = sorted(ans);

        for form in sirius {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_survivor() {
        let jianghsi = get_unit(37);

        let form_abilities = [
            vec![A::Survives { chance: 50 }],
            vec![A::Survives { chance: 50 }],
            vec![A::Survives { chance: 100 }],
        ];

        for (form, ans) in zip(jianghsi, form_abilities) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_bora() {
        let bora = get_unit(359);

        let form_abilities = [
            vec![A::MassiveDamage, A::ImmuneToWarp],
            vec![
                A::Resist,
                A::MassiveDamage,
                A::BarrierBreaker { chance: 100 },
                A::ImmuneToWarp,
            ],
            vec![
                A::Resist,
                A::MassiveDamage,
                A::BarrierBreaker { chance: 100 },
                A::ImmuneToWarp,
            ],
            vec![
                A::Resist,
                A::MassiveDamage,
                A::Weaken {
                    chance: 100,
                    duration: 120,
                    multiplier: 50,
                },
                A::BarrierBreaker { chance: 100 },
                A::ImmuneToWarp,
            ],
        ];

        for (form, ans) in zip(bora, form_abilities) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_mini_wave() {
        let moneko = get_unit(16);

        let form_abilities = [
            vec![A::Crit { chance: 15 }],
            vec![A::Crit { chance: 15 }],
            vec![
                A::Crit { chance: 20 },
                A::Wave(Wave {
                    wtype: WaveType::MiniWave,
                    chance: 100,
                    level: 3,
                }),
            ],
        ];

        for (form, ans) in zip(moneko, form_abilities) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_different_hits() {
        let god = get_unit(437);

        let form_abilities = [
            vec![A::Knockback { chance: 100 }],
            vec![A::Knockback { chance: 100 }],
            vec![A::Knockback { chance: 100 }, A::DoubleBounty],
        ];

        for (form, ans) in zip(god, form_abilities) {
            assert_eq!(form, ans);
        }
    }

    #[test]
    fn test_mini_surge() {
        let dphono = get_unit(705);

        let form_abilities = [
            vec![
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
            ],
            vec![
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
            ],
        ];

        for (form, ans) in zip(dphono, form_abilities) {
            assert_eq!(form, sorted(ans));
        }
    }

    #[test]
    fn test_base_destroyer() {
        let warlock_pierre = get_unit(226);

        let form_abilities = [
            vec![A::TargetsOnly, A::BaseDestroyer],
            vec![A::TargetsOnly, A::DoubleBounty],
            vec![
                A::TargetsOnly,
                A::ImmuneToKB,
                A::ImmuneToFreeze,
                A::ImmuneToSlow,
                A::ImmuneToWeaken,
            ],
        ];

        for (form, ans) in zip(warlock_pierre, form_abilities) {
            assert_eq!(form, sorted(ans));
        }
    }
}
