//! Deals with cat/enemy abilities.

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
    #[rustfmt::skip]
    const fn get_config(&self) -> Config {
        match self {
            Self::StrongAgainst => todo!(),
            Self::Knockback {..} => todo!(),
            Self::Freeze {..} => Config { is_general: false, is_cursable: true },
            Self::Slow {..} => todo!(),
            Self::Resist => todo!(),
            Self::MassiveDamage => todo!(),
            Self::Crit {..} => todo!(),
            Self::TargetsOnly => todo!(),
            Self::DoubleBounty => Config { is_general: true, is_cursable: false },
            Self::BaseDestroyer => todo!(),
            Self::Wave(_) => todo!(),
            Self::Weaken {..} => todo!(),
            Self::Strengthen {..} => todo!(),
            Self::Survives {..} => todo!(),
            Self::Metal => todo!(),
            Self::ImmuneToWave => todo!(),
            Self::WaveBlocker => todo!(),
            Self::ImmuneToKB => todo!(),
            Self::ImmuneToFreeze => todo!(),
            Self::ImmuneToSlow => todo!(),
            Self::ImmuneToWeaken => todo!(),
            Self::ZombieKiller => todo!(),
            Self::WitchKiller1 => todo!(),
            Self::ImmuneToBossShockwave => todo!(),
            Self::Kamikaze => todo!(),
            Self::BarrierBreaker {..} => todo!(),
            Self::ImmuneToWarp => todo!(),
            Self::WitchKiller2 => todo!(),
            Self::ImmuneToCurse => todo!(),
            Self::InsaneResist => todo!(),
            Self::InsaneDamage => todo!(),
            Self::SavageBlow {..} => todo!(),
            Self::Dodge {..} => todo!(),
            Self::Surge(_) => todo!(),
            Self::ImmuneToToxic => todo!(),
            Self::ImmuneToSurge => todo!(),
            Self::Curse {..} => todo!(),
            Self::ShieldPierce {..} => todo!(),
            Self::ColossusSlayer => todo!(),
            Self::Soulstrike => todo!(),
            Self::BehemothSlayer {..} => todo!(),
            Self::CounterSurge => todo!(),
            Self::ConjureUnit {..} => todo!(),
            Self::SageSlayer => todo!(),
            Self::MetalKiller {..} => todo!(),
            Self::Explosion {..} => todo!(),
            Self::ImmuneToExplosion => todo!(),
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
