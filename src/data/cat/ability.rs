//! Deals with cat/enemy abilities.

#![allow(missing_docs, dead_code, unreachable_code)]

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
pub enum WaveType {
    Wave,
    MiniWave,
}

#[derive(Debug)]
pub struct Wave {
    wtype: WaveType,
    chance: Percent,
    level: u8,
}

#[derive(Debug)]
pub enum SurgeType {
    Surge,
    MiniSurge,
}

#[derive(Debug)]
pub struct Surge {
    stype: SurgeType,
    surge_chance: Percent,
    param_0: u16,
    param_1: u16,
    level: u8,
}

#[derive(Debug)]
/// Cat or enemy ability.
pub enum Ability {
    StrongAgainst,
    Knockback {
        chance: Percent,
    },
    /// Freeze the enemy.
    Freeze {
        /// Chance to freeze the enemy.
        chance: Percent,
        /// Duration of freeze in frames.
        duration: u16,
    },
    /// Slow the enemy.
    Slow {
        /// Chance to slow the enemy.
        chance: Percent,
        /// Duration of slow in frames.
        duration: u16,
    },
    Resist,
    MassiveDamage,
    Crit {
        chance: Percent,
    },
    TargetsOnly,
    /// Double money collected when defeating the enemy.
    DoubleBounty,
    BaseDestroyer,
    Wave(Wave),
    Weaken {
        chance: Percent,
        duration: u16,
        multiplier: Percent,
    },
    Strengthen {
        chance: Percent,
        multiplier: u16,
    },
    Survives {
        chance: Percent,
    },
    Metal,
    ImmuneToWave,
    WaveBlocker,
    ImmuneToKB,
    ImmuneToFreeze,
    ImmuneToSlow,
    ImmuneToWeaken,
    ZombieKiller,
    WitchKiller1,
    ImmuneToBossShockwave,
    Kamikaze,
    BarrierBreaker {
        chance: Percent,
    },
    ImmuneToWarp,
    WitchKiller2,
    ImmuneToCurse,
    InsaneResist,
    InsaneDamage,
    SavageBlow {
        chance: Percent,
        /// Additional damage as a percent of initial.
        damage: u16,
    },
    Dodge {
        chance: Percent,
        duration: u16,
    },
    Surge(Surge),
    ImmuneToToxic,
    ImmuneToSurge,
    Curse {
        chance: Percent,
        duration: u16,
    },
    ShieldPierce {
        chance: Percent,
    },
    ColossusSlayer,
    Soulstrike,
    BehemothSlayer {
        dodge_chance: Percent,
        dodge_duration: u16,
    },
    CounterSurge,
    ConjureUnit {
        /// ID of the conjured spirit.
        id: u16,
    },
    SageSlayer,
    MetalKiller {
        damage: Percent,
    },
    Explosion {
        chance: Percent,
        range: u16,
    },
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
                is_general: todo!(),
                is_cursable: true,
            },
            Self::DoubleBounty => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::BaseDestroyer => Config {
                is_general: todo!(),
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
                is_general: todo!(),
                is_cursable: false,
            },
            Self::Survives { .. } => Config {
                is_general: true,
                is_cursable: false,
            },
            Self::Metal => Config {
                is_general: todo!(),
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
            Self::WitchKiller1 => Config {
                is_general: todo!(),
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
            Self::WitchKiller2 => Config {
                is_general: todo!(),
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

// Check Donut's `multab`.
