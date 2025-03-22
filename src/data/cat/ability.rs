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
    WitchKiller1,
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
    /// Witch killer, but it appears twice for some reason.
    WitchKiller2,
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
            Self::WitchKiller1 => Config {
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
            Self::WitchKiller2 => Config {
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
