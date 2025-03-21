//! Deals with cat/enemy abilities.

struct Config {
    is_general: bool,
    is_cursable: bool,
}

#[derive(Debug)]
/// Cat or enemy ability.
pub enum Ability {
    /// Freeze the enemy.
    Freeze {
        /// Chance to freeze the enemy.
        chance: u8,
        /// Duration of freeze in frames.
        duration: u32,
    },
    /// Double money collected when defeating the enemy.
    DoubleBounty,
}
impl Ability {
    const fn get_config(&self) -> Config {
        match self {
            Self::Freeze { .. } => Config {
                is_general: false,
                is_cursable: true,
            },
            Self::DoubleBounty => Config {
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
