//! Deals with cat/enemy abilities.

use super::raw::CombinedCatData;

/// Configuration values/modifiers for abilities.
struct Config {
    /// Does the ability apply on every hit or only on specified ones (e.g.
    /// CgtG's KB vs double bounty).
    is_general: bool,
    /// Is the ability removed by curse.
    is_cursable: bool,
}

type Percent = u8;

#[derive(Debug)]
/// Possible type of wave attack.
pub enum WaveType {
    /// Normal wave.
    Wave,
    /// Mini wave, 2x speed and 20% damage.
    MiniWave,
}

#[derive(Debug)]
/// Wave ability.
pub struct Wave {
    /// Type of wave.
    pub wtype: WaveType,
    /// Chance for wave to proc.
    pub chance: Percent,
    /// Level of wave (amount of hits).
    pub level: u8,
}

#[derive(Debug)]
/// Possible type of surge attack.
pub enum SurgeType {
    /// Normal surge.
    Surge,
    /// Mini surge, 20% damage.
    MiniSurge,
}

#[derive(Debug)]
/// Surge ability.
pub struct Surge {
    /// Type of surge.
    pub stype: SurgeType,
    /// Chance for surge to proc.
    pub surge_chance: Percent,
    /// Surge range min bound (* 4 for some reason).
    pub spawn: u16,
    /// Surge range distance (once again * 4).
    pub range: u16,
    /// Level of surge (20f per level),
    pub level: u8,
}

#[derive(Debug)]
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
        range: u16,
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

fn bool(value: Percent) -> bool {
    match value {
        0 => false,
        1 => true,
        _ => panic!("{value} is not a boolean"),
    }
}

fn chance(value: Percent) -> bool {
    value > 0
}

impl Ability {
    #[allow(unreachable_code)]
    pub fn get_all_abilities((fixed, variable): &CombinedCatData) -> Vec<Ability> {
        let mut abilities = vec![];

        if bool(fixed.has_strong) {
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
                duration: fixed.freeze_time,
            });
        }

        if chance(fixed.slow_chance) {
            abilities.push(Self::Slow {
                chance: fixed.slow_chance,
                duration: fixed.slow_time,
            });
        }

        if bool(fixed.has_resist) {
            abilities.push(Self::Resist);
        }

        if bool(fixed.has_massive_dmg) {
            abilities.push(Self::MassiveDamage);
        }

        if chance(fixed.crit_chance) {
            abilities.push(Self::Crit {
                chance: fixed.crit_chance,
            });
        }

        if bool(fixed.has_targets_only) {
            abilities.push(Self::TargetsOnly);
        }

        if bool(fixed.has_double_bounty) {
            abilities.push(Self::DoubleBounty);
        }

        if bool(fixed.has_double_bounty) {
            abilities.push(Self::BaseDestroyer);
        }

        if chance(todo!()) {
            abilities.push(Self::Wave(_));
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

        if bool(fixed.has_metal) {
            abilities.push(Self::Metal);
        }

        if bool(fixed.immune_wave) {
            abilities.push(Self::ImmuneToWave);
        }

        if bool(fixed.wave_blocker) {
            abilities.push(Self::WaveBlocker);
        }

        if bool(fixed.immune_kb) {
            abilities.push(Self::ImmuneToKB);
        }

        if bool(fixed.immune_freeze) {
            abilities.push(Self::ImmuneToFreeze);
        }

        if bool(fixed.immune_slow) {
            abilities.push(Self::ImmuneToSlow);
        }

        if bool(fixed.immune_weaken) {
            abilities.push(Self::ImmuneToWeaken);
        }

        if bool(variable.has_zkill.unwrap_or_default()) {
            abilities.push(Self::ZombieKiller);
        }

        if bool(variable.has_wkill) {
            abilities.push(Self::WitchKiller);
        }

        if bool(variable.immune_boss_shockwave) {
            abilities.push(Self::ImmuneToBossShockwave);
        }

        if bool(variable.kamikaze) {
            abilities.push(Self::Kamikaze);
        }

        if chance(variable.barrier_break_chance) {
            abilities.push(Self::BarrierBreaker {
                chance: variable.barrier_break_chance,
            });
        }

        if bool(variable.immune_warp) {
            abilities.push(Self::ImmuneToWarp);
        }

        if bool(variable.eva_angel_killer) {
            abilities.push(Self::EvaAngelKiller);
        }

        if bool(variable.immune_curse) {
            abilities.push(Self::ImmuneToCurse);
        }

        if bool(variable.has_insane_resist) {
            abilities.push(Self::InsaneResist);
        }

        if bool(variable.has_insane_damage) {
            abilities.push(Self::InsaneDamage);
        }

        if chance(variable.savage_blow_chance) {
            abilities.push(Self::SavageBlow {
                chance: variable.savage_blow_chance,
                damage: variable.savage_blow_dmg_percent,
            });
        }

        if chance(variable.dodge_chance) {
            abilities.push(Self::Dodge {
                chance: variable.dodge_chance,
                duration: variable.dodge_duration,
            });
        }

        if chance(variable.surge_chance) {
            abilities.push(Self::Surge(todo!()));
        }

        if bool(variable.immune_toxic) {
            abilities.push(Self::ImmuneToToxic);
        }

        if bool(variable.immune_surge) {
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

        if bool(variable.has_colossus_slayer) {
            abilities.push(Self::ColossusSlayer);
        }

        if bool(variable.soulstrike) {
            abilities.push(Self::Soulstrike);
        }

        if bool(variable.has_behemoth_slayer) {
            abilities.push(Self::BehemothSlayer {
                dodge_chance: variable.bslayer_dodge_chance,
                dodge_duration: variable.bslayer_dodge_duration,
            });
        }

        if bool(variable.counter_surge) {
            abilities.push(Self::CounterSurge);
        }

        if variable.conjure_unit > 0 {
            abilities.push(Self::ConjureUnit {
                id: variable.conjure_unit as u16,
            });
        }

        if bool(variable.has_sage_slayer) {
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
                range: variable.explosion_range,
            });
        }

        if bool(variable.immune_explosion) {
            abilities.push(Self::ImmuneToExplosion);
        }

        todo!()
    }
}

/*
Cat
A. Bahamut
no abilities

Kasli -> level 3 surge
Ultra Kaguya
Dr. Nova
Thaumaturge
Li'l Valk
Nobiluga
GothMit
Lasvoss
Doron
Iz of mourning
Bombercat all forms
Courier
Cosmo ultra
Matador
soulstrike
Colossus
Jianghsi
sblow
Bora all forms
Something with wave and mini wave
Thief/cat jobs
CGtG
*/
